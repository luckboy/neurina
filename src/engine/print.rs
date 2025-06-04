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

pub trait Print
{
    fn print_pv(&self, w: &mut dyn Write, board: &Board, depth: usize, value: i32, time: Duration, node_count: u64, pv: &[Move]) -> Result<()>;
    
    fn print_best_move(&self, w: &mut dyn Write, board: &Board, mv: Move) -> Result<()>;
    
    fn print_outcome(&self, w: &mut dyn Write, outcome: Outcome) -> Result<()>;
}

#[derive(Copy, Clone, Debug)]
pub struct EmptyPrinter;

impl EmptyPrinter
{
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
