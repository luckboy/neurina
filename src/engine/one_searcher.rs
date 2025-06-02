//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use crate::chess::movegen::semilegal;
use crate::chess::types::OutcomeFilter;
use crate::chess::Move;
use crate::chess::MoveChain;
use crate::chess::Outcome;
use crate::engine::eval::*;
use crate::engine::middle_searcher::*;
use crate::engine::search::*;
use crate::shared::intr_check::*;
use crate::shared::Interruption;

#[derive(Clone)]
pub struct OneSearcher
{
    middle_searcher: MiddleSearcher,
    middle_depth: usize,
}

impl OneSearcher
{
    pub fn new(middle_searcher: MiddleSearcher, middle_depth: usize) -> Self
    { OneSearcher { middle_searcher, middle_depth, } }
    
    pub fn middle_searcher(&self) -> &MiddleSearcher
    { &self.middle_searcher }
}

impl Search for OneSearcher
{
    fn intr_checker(&self) -> &Arc<dyn IntrCheck>
    { self.middle_searcher.intr_checker() }
    
    fn search(&self, move_chain: &mut MoveChain, depth: usize, search_moves: &Option<Vec<Move>>) -> Result<(i32, u64, u64, Vec<Move>), Interruption>
    {
        let moves = semilegal::gen_all(move_chain.last());
        let mut middle_node_count = 1u64;
        let mut node_count = 1u64;
        let mut pv: Vec<Move> = Vec::new();
        let mut best_value = MIN_EVAL_VALUE;
        match move_chain.set_auto_outcome(OutcomeFilter::Relaxed) {
            Some(outcome) => {
                let value = match outcome {
                    Outcome::Win { .. } => MIN_EVAL_ROOT_MATE_VALUE,
                    Outcome::Draw(_) => 0,
                };
                return Ok((value, middle_node_count, node_count, pv));
            },
            None => (),
        }
        move_chain.clear_outcome();
        for mv in &moves {
            match search_moves {
                Some(search_moves) => {
                    match search_moves.iter().find(|mv2| *mv2 == mv) {
                        Some(_) => (),
                        None => continue,
                    }
                },
                None => (),
            }
            match move_chain.push(*mv) {
                Ok(()) => {
                    match move_chain.set_auto_outcome(OutcomeFilter::Relaxed) {
                        Some(outcome) => {
                            let value = match outcome {
                                Outcome::Win { .. } => MAX_EVAL_MIDDLE_MATE_VALUE,
                                Outcome::Draw(_) => 0,
                            };
                            if value >= best_value {
                                best_value = value;
                                pv = vec![*mv];
                            }
                            node_count += 1;
                            move_chain.pop();
                            continue;
                        },
                        None => (),
                    }
                    let res = self.middle_searcher.search(move_chain.last(), self.middle_depth, depth - 1);
                    match res {
                        Ok((neg_value, tmp_middle_node_count, tmp_node_count, tmp_pv)) => {
                            let value = -neg_value;
                            if value >= best_value {
                                best_value = value;
                                pv = vec![*mv];
                                pv.extend_from_slice(tmp_pv.as_slice());
                            }
                            middle_node_count += tmp_middle_node_count;
                            node_count += tmp_node_count;
                        },
                        Err(intr) => {
                            move_chain.pop();
                            return Err(intr);
                        },
                    }
                    move_chain.pop();
                },
                Err(_) => (),
            }
        }
        Ok((best_value, middle_node_count, node_count, pv))
    }

    fn move_count_to_checkmate(&self, value: i32, depth: usize) -> Option<usize>
    {
        if value >= MAX_EVAL_ROOT_MATE_VALUE {
            let max_value = MAX_EVAL_ROOT_MATE_VALUE;
            let start_move_count = 0;
            Some((max_value - value + start_move_count) as usize)
        } else if value >= MAX_EVAL_MIDDLE_MATE_VALUE {
            let max_value = MAX_EVAL_MIDDLE_MATE_VALUE + (self.middle_depth as i32);
            let start_move_count = 1;
            Some((max_value - value + start_move_count) as usize)
        } else if value >= MAX_EVAL_MATE_VALUE {
            let max_value = MAX_EVAL_MATE_VALUE + ((depth - self.middle_depth - 1) as i32);
            let start_move_count = (self.middle_depth as i32) + 1;
            Some((max_value - value + start_move_count) as usize)
        } else {
            None
        }
    }

    fn min_depth(&self) -> usize
    { 1 + self.middle_depth + 1 }
}

#[cfg(test)]
mod tests;
