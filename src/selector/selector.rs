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
use std::sync::Mutex;
use rand::random_range;
use crate::selector::lichess_puzzles::*;
use crate::selector::print::*;
use crate::shared::lichess_puzzle::*;

pub struct Selector
{
    writer: Arc<Mutex<dyn Write + Send + Sync>>,
    printer: Arc<dyn Print + Send + Sync>,
}

impl Selector
{
    pub const PUZZLE_COUNT_TO_PRINT: u64 = 64 * 1024;
    
    
    pub fn new(writer: Arc<Mutex<dyn Write + Send + Sync>>, printer: Arc<dyn Print + Send + Sync>) -> Self
    { Selector { writer, printer, } }

    pub fn writer(&self) -> &Arc<Mutex<dyn Write + Send + Sync>>
    { &self.writer }

    pub fn printer(&self) -> &Arc<dyn Print + Send + Sync>
    { &self.printer }
    
    pub fn select(&self, puzzles: &mut dyn Iterator<Item = Result<LichessPuzzle>>, writer: &mut dyn Write, divider: u64) -> Result<()>
    {
        let mut puzzle_writer = LichessPuzzleWriter::from_writer(writer);
        let mut puzzle_count = 0u64;
        let mut i = 0u64;
        for puzzle in puzzles {
            let puzzle = puzzle?;
            if puzzle_count % Self::PUZZLE_COUNT_TO_PRINT == 0 {
                let mut writer_g = self.writer.lock().unwrap();
                self.printer.print(&mut *writer_g, puzzle_count, false)?;
                writer_g.flush()?;
            }
            if puzzle_count % divider == 0 {
                i = random_range(0..divider);
            }
            if puzzle_count % divider == i {
                puzzle_writer.write_puzzle(&puzzle)?;
            }
            puzzle_count += 1;
        }
        {
            let mut writer_g = self.writer.lock().unwrap();
            self.printer.print(&mut *writer_g, puzzle_count, true)?;
            writer_g.flush()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
