//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::mem::swap;
use std::sync::Arc;
use crate::chess::movegen::semilegal;
use crate::chess::Board;
use crate::chess::Move;
use crate::chess::MoveList;
use crate::engine::eval::*;
use crate::engine::neural_search::*;
use crate::shared::intr_check::*;
use crate::shared::Interruption;

/// A structure of middle searcher.
///
/// The middle search is a search of game tree that is between a classical tree search and a neural
/// search.
#[derive(Clone)]
pub struct MiddleSearcher
{
    eval_fun: Arc<dyn Eval + Send + Sync>,
    neural_searcher: Arc<dyn NeuralSearch + Send + Sync>,
}

impl MiddleSearcher
{
    /// A number of nodes to check interruption.
    pub const NODE_COUNT_TO_INTR_CHECK: u64 = 1024;

    /// Creates a a middle searcher.
    pub fn new(eval_fun: Arc<dyn Eval + Send + Sync>, neural_searcher: Arc<dyn NeuralSearch + Send + Sync>) -> Self
    { MiddleSearcher { eval_fun, neural_searcher, } }

    /// Returns the interruption checker.
    pub fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>
    { self.neural_searcher.intr_checker() }
    
    /// Returns the evaluation function.
    pub fn eval_fun(&self) -> &Arc<dyn Eval + Send + Sync>
    { &self.eval_fun }

    /// Returns the neural searcher.
    pub fn neural_searcher(&self) -> &Arc<dyn NeuralSearch + Send + Sync>
    { &self.neural_searcher }
    
    fn nega_max_with_fun_ref<F>(&self, board: &Board, current_pv: &mut Vec<Move>, pvs: &mut [Vec<Move>], node_count: &mut u64, leaf_count: &mut usize, ply: usize, middle_depth: usize, f: &mut F) -> Result<(i32, Option<usize>), Interruption>
        where F: FnMut(&Board, &[Move], usize) -> i32
    {
        if *node_count % Self::NODE_COUNT_TO_INTR_CHECK == 0 {
            self.neural_searcher.intr_checker().check()?;
        }
        *node_count += 1;
        if middle_depth <= 0 {
            let leaf_idx = *leaf_count;
            pvs[ply] = Vec::new();
            if !board.has_legal_moves() {
                if board.is_check() {
                    Ok((MIN_EVAL_MIDDLE_MATE_VALUE, None))
                } else {
                    Ok((0, None))
                }
            } else {
                *leaf_count += 1;
                Ok((f(board, current_pv.as_slice(), leaf_idx), Some(leaf_idx)))
            }
        } else {
            let mut moves: MoveList = MoveList::new();
            let mut are_moves = false;
            let mut best_value = MIN_EVAL_VALUE;
            let mut best_leaf_idx = None;
            semilegal::gen_all_into(board, &mut moves);
            for mv in &moves {
                match board.make_move(*mv) {
                    Ok(new_board) => {
                        current_pv.push(*mv);
                        let (neg_value, leaf_idx) = self.nega_max_with_fun_ref(&new_board, current_pv, pvs, node_count, leaf_count, ply + 1, middle_depth - 1, f)?;
                        let value = -neg_value;
                        current_pv.pop();
                        if value > best_value {
                            best_value = value;
                            best_leaf_idx = leaf_idx;
                            let mut pv: Vec<Move> = Vec::new();
                            swap(&mut pv, &mut pvs[ply]);
                            pv.clear();
                            pv.push(*mv);
                            pv.extend_from_slice(pvs[ply + 1].as_slice());
                            swap(&mut pv, &mut pvs[ply]);
                        }
                        are_moves = true;
                    },
                    Err(_) => (),
                }
            }
            if !are_moves {
                if board.is_check() {
                    Ok((MIN_EVAL_MIDDLE_MATE_VALUE - (middle_depth as i32),  None))
                } else {
                    Ok((0, None))
                }
            } else {
                Ok((best_value, best_leaf_idx))
            }
        }
    }

    fn nega_max<F>(&self, board: &Board, current_pv: &mut Vec<Move>, pvs: &mut [Vec<Move>], node_count: &mut u64, leaf_count: &mut usize, ply: usize, middle_depth: usize, mut f: F) -> Result<(i32, Option<usize>), Interruption>
        where F: FnMut(&Board, &[Move], usize) -> i32
    { self.nega_max_with_fun_ref(board, current_pv, pvs, node_count, leaf_count, ply, middle_depth, &mut f) }

    /// Searches a game tree from the board.
    ///
    /// This method returns a value, a number of nodes of middle search, a number of nodes, and
    /// a principal variation.
    pub fn search(&self, board: &Board, middle_depth: usize, depth: usize) -> Result<(i32, u64, u64, Vec<Move>), Interruption>
    {
        let mut current_pv: Vec<Move> = Vec::new();
        let mut pvs: Vec<Vec<Move>> = vec![Vec::with_capacity(middle_depth); middle_depth + 1];
        let mut neural_pvs: Vec<Vec<Move>> = Vec::new();
        let mut middle_node_count = 1u64;
        let mut leaf_count = 0usize;
        let (value, _) = self.nega_max(board, &mut current_pv, pvs.as_mut_slice(), &mut middle_node_count, &mut leaf_count, 0, middle_depth, |_, pv, _| {
                let mut neural_pv: Vec<Move> = Vec::with_capacity(depth);
                neural_pv.extend_from_slice(pv);
                neural_pvs.push(neural_pv);
                0
        })?;
        if value <= MIN_EVAL_MATE_VALUE || value >= MAX_EVAL_MATE_VALUE || neural_pvs.is_empty() {
            return Ok((value, middle_node_count, middle_node_count, pvs[0].clone()));
        }
        self.neural_searcher.search(board, &mut neural_pvs, depth - middle_depth)?;
        let mut neural_node_count = 0u64;
        pvs = vec![Vec::new(); middle_depth + 1];
        middle_node_count += 1;
        leaf_count = 0usize;
        let (value, leaf_idx) = self.nega_max(board, &mut current_pv, pvs.as_mut_slice(), &mut middle_node_count, &mut leaf_count, 0, middle_depth, |new_board, _, leaf_idx| {
                let mut tmp_board = new_board.clone();
                let mut neural_ply = 0usize;
                for mv in &neural_pvs[leaf_idx][middle_depth..] {
                    match tmp_board.make_move(*mv) {
                        Ok(tmp_new_board) => tmp_board = tmp_new_board,
                        Err(_) => break,
                    }
                    neural_node_count += 1;
                    neural_ply += 1;
                }
                let value = if !tmp_board.has_legal_moves() {
                    if tmp_board.is_check() {
                        if neural_ply > 0 {
                            MIN_EVAL_MATE_VALUE - ((depth - middle_depth - neural_ply) as i32)
                        } else {
                            MIN_EVAL_MIDDLE_MATE_VALUE
                        }
                    } else {
                        0
                    }
                } else {
                    self.eval_fun.evaluate(&tmp_board)
                };
                if neural_ply % 2 == 0 {
                    value
                } else {
                    -value
                }
        })?;
        let node_count = middle_node_count + neural_node_count;
        match leaf_idx {
            Some(leaf_idx) => Ok((value, middle_node_count, node_count, neural_pvs[leaf_idx].clone())),
            None => Ok((value, middle_node_count, node_count, pvs[0].clone())),
        }
    }
}

#[cfg(test)]
mod tests;
