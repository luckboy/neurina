//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use std::io::Write;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use crate::chess::types::OutcomeFilter;
use crate::chess::Move;
use crate::chess::MoveChain;
use crate::engine::print::*;
use crate::engine::search::*;
use crate::engine::syzygy::*;
use crate::shared::intr_check::*;

pub struct Thinker
{
    searcher: Arc<dyn Search + Send + Sync>,
    writer: Arc<Mutex<dyn Write + Send + Sync>>,
    printer: Arc<dyn Print + Send + Sync>,
    syzygy: Arc<Mutex<Option<Syzygy>>>,
    is_stopped: Mutex<bool>,
    condvar: Condvar,
}

impl Thinker
{
    pub fn new(searcher: Arc<dyn Search + Send + Sync>, writer: Arc<Mutex<dyn Write + Send + Sync>>, printer: Arc<dyn Print  + Send + Sync>, syzygy: Arc<Mutex<Option<Syzygy>>>) -> Self
    {
        Thinker {
            searcher,
            writer,
            printer,
            syzygy,
            is_stopped: Mutex::new(true),
            condvar: Condvar::new(),
        }
    }

    pub fn searcher(&self) -> &Arc<dyn Search + Send + Sync>
    { &self.searcher }
    
    pub fn writer(&self) -> &Arc<Mutex<dyn Write + Send + Sync>>
    { &self.writer }
    
    pub fn printer(&self) -> &Arc<dyn Print  + Send + Sync>
    { &self.printer }
    
    pub fn syzygy(&self) -> &Arc<Mutex<Option<Syzygy>>>
    { &self.syzygy }
    
    pub fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>
    { self.searcher.intr_checker() }

    pub fn start(&self)
    {
        {
            let mut is_stopped_g = self.is_stopped.lock().unwrap();
            *is_stopped_g = false;
        }
        self.searcher.intr_checker().start();
    }

    pub fn wait(&self)
    {
        let mut is_stopped_g = self.is_stopped.lock().unwrap();
        while !*is_stopped_g {
            is_stopped_g = self.condvar.wait(is_stopped_g).unwrap();
        }
    }
    
    pub fn stop(&self)
    {
        let mut is_stopped_g = self.is_stopped.lock().unwrap();
        *is_stopped_g = true;
        self.condvar.notify_one();
    }
    
    pub fn is_stopped(&self) -> bool
    {
        let is_stopped_g = self.is_stopped.lock().unwrap();
        *is_stopped_g
    }
    
    pub fn think(&self, move_chain: &Arc<Mutex<MoveChain>>, search_moves: &Option<Vec<Move>>, max_depth: Option<usize>, max_node_count: Option<u64>, move_count_to_checkmate: Option<usize>, now: Instant, timeout: Option<Duration>, can_make_best_move: bool, can_print_pv: bool, can_print_best_move_and_outcome: bool) -> Result<()>
    {
        {
            let mut move_chain_g = move_chain.lock().unwrap();
            let mut depth = self.searcher.min_depth();
            match timeout {
                Some(timeout) => {
                    self.searcher.intr_checker().set_timeout(now, timeout);
                },
                None => {
                    self.searcher.intr_checker().unset_timeout();
                },
            }
            let mut best_move: Option<Move> = None;
            {
                let mut syzygy_g = self.syzygy.lock().unwrap();
                match &mut *syzygy_g {
                    Some(syzygy) => best_move = syzygy.probe(move_chain_g.last()),
                    None => (),
                }
            }
            if best_move.is_none() {
                let mut is_first = true;
                let mut node_count = 0u64; 
                loop {
                    self.searcher.intr_checker().set_first(is_first);
                    match self.searcher.search(&mut *move_chain_g, depth, search_moves) {
                        Ok((value, _, search_node_count, pv)) => {
                            best_move = pv.first().map(|mv| *mv);
                            node_count += search_node_count;
                            if can_print_pv {
                                let mut writer_g = self.writer.lock().unwrap();
                                self.printer.print_pv(&mut *writer_g, move_chain_g.last(), depth, value, now.elapsed(), node_count, pv.as_slice())?;
                                writer_g.flush()?;
                            }
                            match max_depth {
                                Some(max_depth) if depth + 1 > max_depth =>  break,
                                _ => (),
                            }
                            match max_node_count {
                                Some(max_node_count) if node_count >= max_node_count =>  break,
                                _ => (),
                            }
                            match move_count_to_checkmate {
                                Some(move_count_to_checkmate) if self.searcher.move_count_to_checkmate(value, depth).map(|n| n <= move_count_to_checkmate * 2).unwrap_or(false) =>  break,
                                _ => (),
                            }
                        },
                        Err(_) => break,
                    }
                    depth += 1;
                    is_first = false;
                }
            }
            if can_print_best_move_and_outcome {
                match best_move {
                    Some(mv) => {
                        let mut writer_g = self.writer.lock().unwrap();
                        self.printer.print_best_move(&mut *writer_g, move_chain_g.last(), mv)?;
                        writer_g.flush()?;
                    },
                    None => (),
                }
            }
            if can_make_best_move {
                match best_move {
                    Some(mv) => {
                        move_chain_g.push(mv).unwrap();
                        let outcome = move_chain_g.set_auto_outcome(OutcomeFilter::Relaxed);
                        move_chain_g.clear_outcome();
                        if can_print_best_move_and_outcome {
                            match outcome {
                                Some(outcome) => {
                                    let mut writer_g = self.writer.lock().unwrap();
                                    self.printer.print_outcome(&mut *writer_g, outcome)?;
                                    writer_g.flush()?;
                                },
                                None => (),
                            }
                        }
                    },
                    None => (),
                }
            }
        }
        self.stop();
        Ok(())
    }
}

#[cfg(test)]
mod tests;
