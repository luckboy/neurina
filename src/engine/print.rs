//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use std::io::Write;
use std::time::Duration;
use crate::chess::Board;
use crate::chess::Move;
use crate::chess::Outcome;

/// A printer trait.
///
/// This trait provides methods which print a line of principal variation, a best move, and a game
/// outcome.
pub trait Print
{
    /// Prints line of principal variation from the depth, the value, the time, the nodes, and
    /// the principal variation.
    fn print_pv(&self, w: &mut dyn Write, board: &Board, depth: usize, value: i32, time: Duration, node_count: u64, pv: &[Move]) -> Result<()>;
    
    /// Prints the best move.
    fn print_best_move(&self, w: &mut dyn Write, board: &Board, mv: Move) -> Result<()>;
    
    /// Prints the game outcome.
    fn print_outcome(&self, w: &mut dyn Write, outcome: Outcome) -> Result<()>;
}

/// A structure of empty printer.
///
/// The empty printer is dummy that doesn't print anything.
#[derive(Copy, Clone, Debug)]
pub struct EmptyPrinter;

impl EmptyPrinter
{
    /// Creates an empty printer.
    pub fn new() -> Self
    { EmptyPrinter }
}

impl Print for EmptyPrinter
{
    fn print_pv(&self, _w: &mut dyn Write, _board: &Board, _depth: usize, _value: i32, _time: Duration, _node_count: u64, _pv: &[Move]) -> Result<()>
    { Ok(()) }
    
    fn print_best_move(&self, _w: &mut dyn Write, _board: &Board, _mv: Move) -> Result<()>
    { Ok(()) }
    
    fn print_outcome(&self, _w: &mut dyn Write, _outcome: Outcome) -> Result<()>
    { Ok(()) }
}
