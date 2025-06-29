//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use std::thread::JoinHandle;
use std::thread::spawn;
use crate::chess::types::OutcomeFilter;
use crate::chess::Move;
use crate::chess::MoveChain;
use crate::engine::print::*;
use crate::engine::thinker::*;

#[derive(Copy, Clone, Debug)]
pub enum TimeControl
{
    Level(usize, Duration),
    Fixed(Duration),
}

#[derive(Clone, Debug)]
struct ThinkingParams
{
    search_moves: Option<Vec<Move>>,
    depth: Option<usize>,
    node_count: Option<u64>,
    move_count_to_checkmate: Option<usize>,
    now: Instant,
    timeout: Option<Duration>,
    can_make_best_move: bool,
    can_print_pv: bool,
    can_print_best_move_and_outcome: bool,
}

#[derive(Clone, Debug)]
enum ThreadCommand
{
    Think(ThinkingParams),
    Quit,
}

pub struct Engine
{
    thread: JoinHandle<()>,
    sender: Sender<ThreadCommand>,
    thinker: Arc<Thinker>,
    move_chain: Arc<Mutex<MoveChain>>,
    time_control: TimeControl,
    remaining_time: Duration,
    move_count_to_go: usize,
}

impl Engine
{
    pub fn new(thinker: Arc<Thinker>) -> Self
    {
        let move_chain = Arc::new(Mutex::new(MoveChain::new_initial()));
        let (sender, receiver) = channel::<ThreadCommand>();
        let thread_thinker = thinker.clone();
        let thread_move_chain = move_chain.clone();
        let thread = spawn(move || {
                loop {
                    match receiver.recv().unwrap() {
                        ThreadCommand::Think(params) => {
                            match thread_thinker.think(&thread_move_chain, &params.search_moves, params.depth, params.node_count, params.move_count_to_checkmate, params.now, params.timeout, params.can_make_best_move, params.can_print_pv, params.can_print_best_move_and_outcome) {
                                Ok(()) => (),
                                Err(err) => {
                                    thread_thinker.stop();
                                    eprintln!("I/O error: {}", err);
                                },
                            }
                        },
                        ThreadCommand::Quit => break,
                    }
                }
        });
        Engine {
            thread,
            sender,
            thinker,
            move_chain,
            time_control: TimeControl::Level(0, Duration::ZERO),
            remaining_time: Duration::from_secs(5 * 60),
            move_count_to_go: 0,
        }
    }
    
    pub fn thinker(&self) -> &Arc<Thinker>
    { &self.thinker }

    pub fn move_chain(&self) -> &Arc<Mutex<MoveChain>>
    { &self.move_chain }
    
    pub fn time_control(&self) -> TimeControl
    { self.time_control }

    pub fn set_time_control(&mut self, time_control: TimeControl)
    { self.time_control = time_control; }

    pub fn remaining_time(&self) -> Duration
    { self.remaining_time }

    pub fn set_remaining_time(&mut self, remaining_time: Duration)
    { self.remaining_time = remaining_time; }

    pub fn move_count_to_go(&self) -> usize
    { self.move_count_to_go }

    pub fn set_move_count_to_go(&mut self, move_count_to_go: usize)
    { self.move_count_to_go = move_count_to_go; }

    pub fn printer(&self) -> &Arc<dyn Print + Send + Sync>
    { self.thinker.printer() }
    
    pub fn do_move_chain<T, F>(&self, f: F) -> T
        where F: FnOnce(&mut MoveChain) -> T
    {
        self.thinker.wait();
        let mut move_chain_g = self.move_chain.lock().unwrap();
        f(&mut *move_chain_g)
    }
    
    pub fn stop(&self)
    { self.thinker.intr_checker().stop(); }
    
    pub fn is_stopped(&self) -> bool
    { self.thinker.is_stopped() } 
    
    fn calculate_timeout(&self) -> Duration
    {
        match self.time_control {
            TimeControl::Level(mps, inc) => {
                let move_count_to_go = if self.move_count_to_go > 0 {
                    self.move_count_to_go
                } else {
                    if mps > 0 {
                        let move_chain_g = self.move_chain.lock().unwrap();
                        mps - (move_chain_g.len() / 2) % mps
                    } else {
                        30
                    }
                };
                let mut timeout = self.remaining_time / (move_count_to_go as u32) + inc / 2;
                if timeout >= self.remaining_time {
                    if self.remaining_time > Duration::from_millis(500) {
                        timeout = self.remaining_time - Duration::from_millis(500);
                    } else {
                        timeout = Duration::ZERO;
                    }
                }
                timeout
            },
            TimeControl::Fixed(time) => {
                if time > Duration::from_millis(500) {
                    time - Duration::from_millis(500)
                } else {
                    Duration::ZERO
                }
            },
        }
    }
    
    pub fn go(&self, search_moves: Option<Vec<Move>>, depth: Option<usize>, node_count: Option<u64>, move_count_to_checkmate: Option<usize>, is_timeout: bool, can_make_best_move: bool, can_print_pv: bool, can_print_best_move_and_outcome: bool)
    {
        self.stop();
        self.thinker.wait();
        let is_outcome = {
            let mut move_chain_g = self.move_chain.lock().unwrap();
            let outcome = move_chain_g.set_auto_outcome(OutcomeFilter::Relaxed);
            move_chain_g.clear_outcome();
            outcome.is_some()
        };
        if !is_outcome {
            self.thinker.start();
            let timeout = if is_timeout {
                Some(self.calculate_timeout())
            } else {
                None
            };
            let params = ThinkingParams {
                search_moves,
                depth, node_count, 
                move_count_to_checkmate,
                now: Instant::now(),
                timeout,
                can_make_best_move,
                can_print_pv,
                can_print_best_move_and_outcome,
            };
            self.sender.send(ThreadCommand::Think(params)).unwrap();
        }
    }
    
    pub fn quit(&self)
    {
        self.stop();
        self.thinker.wait();
        self.sender.send(ThreadCommand::Quit).unwrap();
    }
    
    pub fn join_thread(self)
    { self.thread.join().unwrap(); }
}

#[cfg(test)]
mod tests;
