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

/// An enumeration of time control.
#[derive(Copy, Clone, Debug)]
pub enum TimeControl
{
    /// A level time control contains the number of moves per time control and the time.
    Level(usize, Duration),
    /// A fixed time control contains the time on one move.
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

/// An engine structure.
///
/// The engine controls a game, a time, and an iterative search. The iterative search is executed
/// in other thread.
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
    /// Creates an engine.
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
    
    /// Returns the thinker.
    pub fn thinker(&self) -> &Arc<Thinker>
    { &self.thinker }

    /// Returns the move chain.
    pub fn move_chain(&self) -> &Arc<Mutex<MoveChain>>
    { &self.move_chain }
    
    /// Returns the time control.
    pub fn time_control(&self) -> TimeControl
    { self.time_control }

    /// Sets the time control.
    pub fn set_time_control(&mut self, time_control: TimeControl)
    { self.time_control = time_control; }

    /// Returns the remaining time.
    pub fn remaining_time(&self) -> Duration
    { self.remaining_time }

    /// Sets the remaining time.
    pub fn set_remaining_time(&mut self, remaining_time: Duration)
    { self.remaining_time = remaining_time; }

    /// Returns the number of move to go.
    pub fn move_count_to_go(&self) -> usize
    { self.move_count_to_go }

    /// Sets the number of move to go.
    pub fn set_move_count_to_go(&mut self, move_count_to_go: usize)
    { self.move_count_to_go = move_count_to_go; }

    /// Returns the printer.
    pub fn printer(&self) -> &Arc<dyn Print + Send + Sync>
    { self.thinker.printer() }
    
    /// Operates on the move chain.
    ///
    /// This method waits for thinker before the operation on the move chain.
    pub fn do_move_chain<T, F>(&self, f: F) -> T
        where F: FnOnce(&mut MoveChain) -> T
    {
        self.thinker.wait();
        let mut move_chain_g = self.move_chain.lock().unwrap();
        f(&mut *move_chain_g)
    }
    
    /// Stops an iterative search.
    pub fn stop(&self)
    { self.thinker.intr_checker().stop(); }
    
    /// Returns `true` if an iterative search is stopped, otherwise `false`.
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
    
    /// Iteratively searchs a game tree.
    ///
    /// The search moves are moves from which the search begins. The maximal depth and the maximal
    /// nodes are the limitations of iterative search. This method searches for a checkmate in the
    /// moves if these moves is specified. The timeout flag determites whether timeout should be
    /// calculated and used. Other flags infrom this method whether it should make a best move,
    /// print a principal variation, and print the best move and an outcome. This method stops an
    /// iterative search and waits for the thinker before the iterative search.
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
    
    /// Quits from the engine.
    ///
    /// This method stops an iterative search, waits for the thinker, and sends the exit message to
    /// the thread.
    pub fn quit(&self)
    {
        self.stop();
        self.thinker.wait();
        self.sender.send(ThreadCommand::Quit).unwrap();
    }
    
    /// Wait for the thread to finish.
    pub fn join_thread(self)
    { self.thread.join().unwrap(); }
}

#[cfg(test)]
mod tests;
