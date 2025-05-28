//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use std::sync::Mutex;
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
    
    fn search(&self, move_chain: &Arc<Mutex<MoveChain>>, depth: usize) -> Result<(i32, u64, Vec<Move>), Interruption>
    {
        let mut move_chain_g = move_chain.lock().unwrap();
        let moves = semilegal::gen_all(move_chain_g.last());
        let mut node_count = 1u64;
        let mut pv: Vec<Move> = Vec::new();
        let mut best_value = MIN_EVAL_VALUE;
        match move_chain_g.set_auto_outcome(OutcomeFilter::Relaxed) {
            Some(outcome) => {
                let value = match outcome {
                    Outcome::Win { .. } => MIN_EVAL_ROOT_MATE_VALUE,
                    Outcome::Draw(_) => 0,
                };
                return Ok((value, node_count, pv));
            },
            None => (),
        }
        move_chain_g.clear_outcome();
        for mv in &moves {
            match move_chain_g.push(*mv) {
                Ok(()) => {
                    match move_chain_g.set_auto_outcome(OutcomeFilter::Relaxed) {
                        Some(outcome) => {
                            let value = match outcome {
                                Outcome::Win { .. } => MIN_EVAL_MIDDLE_MATE_VALUE,
                                Outcome::Draw(_) => 0,
                            };
                            if value >= best_value {
                                best_value = value;
                                pv = vec![*mv];
                            }
                            node_count += 1;
                            move_chain_g.pop();
                            continue;
                        },
                        None => (),
                    }
                    let res = self.middle_searcher.search(move_chain_g.last(), self.middle_depth, depth - 1);
                    match res {
                        Ok((neg_value, tmp_node_count, tmp_pv)) => {
                            let value = -neg_value;
                            if value >= best_value {
                                best_value = value;
                                pv = vec![*mv];
                                pv.extend_from_slice(tmp_pv.as_slice());
                            }
                            node_count += tmp_node_count;
                        },
                        Err(intr) => {
                            move_chain_g.pop();
                            return Err(intr);
                        },
                    }
                    move_chain_g.pop();
                },
                Err(_) => (),
            }
        }
        Ok((best_value, node_count, pv))
    }
}

#[cfg(test)]
mod tests;
