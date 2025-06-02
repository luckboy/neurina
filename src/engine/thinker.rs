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
use crate::shared::intr_check::*;

pub struct Thinker
{
    searcher: Arc<dyn Search>,
    writer: Arc<Mutex<dyn Write>>,
    printer: Arc<dyn Print>,
    is_stopped: Mutex<bool>,
    condvar: Condvar,
}

impl Thinker
{
    pub fn new(searcher: Arc<dyn Search>, writer: Arc<Mutex<dyn Write>>) -> Self
    {
        Thinker {
            searcher,
            writer,
            printer: Arc::new(EmptyPrinter::new()),
            is_stopped: Mutex::new(true),
            condvar: Condvar::new(),
        }
    }

    pub fn intr_checker(&self) -> &Arc<dyn IntrCheck>
    { self.searcher.intr_checker() }
    
    pub fn set_printer(&mut self, printer: Arc<dyn Print>)
    { self.printer = printer; }
    
    pub fn start(&self)
    {
        let mut is_stopped_g = self.is_stopped.lock().unwrap();
        *is_stopped_g = false;
    }

    pub fn wait(&self)
    {
        let mut is_stopped_g = self.is_stopped.lock().unwrap();
        while !*is_stopped_g {
            is_stopped_g = self.condvar.wait(is_stopped_g).unwrap();
        }
    }
    
    fn stop(&self)
    {
        let mut is_stopped_g = self.is_stopped.lock().unwrap();
        *is_stopped_g = true;
        self.condvar.notify_one();
    }
    
    pub fn think(&self, move_chain: &Arc<Mutex<MoveChain>>, search_moves: &Option<Vec<Move>>, max_depth: Option<usize>, max_node_count: Option<u64>, checkmate_move_count: Option<usize>, timeout: Option<Duration>, can_make_best_move: bool, can_print_pv: bool, can_print_best_move_and_outcome: bool) -> Result<()>
    {
        {
            let now = Instant::now();
            let mut move_chain_g = move_chain.lock().unwrap();
            let mut depth = self.searcher.min_depth();
            self.searcher.intr_checker().start();
            match timeout {
                Some(timeout) => {
                    self.searcher.intr_checker().set_timeout(now, timeout);
                },
                None => {
                    self.searcher.intr_checker().unset_timeout();
                },
            }
            let mut is_first = true;
            let mut best_move: Option<Move> = None;
            if best_move.is_none() {
                loop {
                    self.searcher.intr_checker().set_first(is_first);
                    match self.searcher.search(&mut *move_chain_g, depth, search_moves) {
                        Ok((value, _, node_count, pv)) => {
                            best_move = pv.first().map(|mv| *mv);
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
                            match checkmate_move_count {
                                Some(checkmate_move_count) if self.searcher.move_count_to_checkmate(value, depth).map(|n| n <= checkmate_move_count * 2).unwrap_or(false) =>  break,
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
            self.searcher.intr_checker().stop();
        }
        self.stop();
        Ok(())
    }
}
