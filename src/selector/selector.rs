//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use rand::random_range;
use crate::selector::lichess_puzzles::*;
use crate::selector::print::*;
use crate::selector::SelectorError;
use crate::selector::SelectorResult;
use crate::shared::intr_check::*;
use crate::shared::lichess_puzzle::*;

/// A selector structure.
///
/// The selector randomly selects Lichess puzzles.
pub struct Selector
{
    intr_checker: Arc<dyn IntrCheck + Send + Sync>,
    writer: Arc<Mutex<dyn Write + Send + Sync>>,
    printer: Arc<dyn Print + Send + Sync>,
}

impl Selector
{
    /// A number of puzzles to print.
    pub const PUZZLE_COUNT_TO_PRINT: u64 = 64 * 1024;
    
    /// Creates a selector.
    pub fn new(intr_checker: Arc<dyn IntrCheck + Send + Sync>, writer: Arc<Mutex<dyn Write + Send + Sync>>, printer: Arc<dyn Print + Send + Sync>) -> Self
    { Selector { intr_checker, writer, printer, } }

    /// Returns the interruption checker.
    pub fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>
    { &self.intr_checker }
    
    /// Returns the writer.
    pub fn writer(&self) -> &Arc<Mutex<dyn Write + Send + Sync>>
    { &self.writer }

    /// Returns the printer.
    pub fn printer(&self) -> &Arc<dyn Print + Send + Sync>
    { &self.printer }
    
    /// Randomly selects the Lichess puzzles.
    ///
    /// The Lichess puzzles are divided into parts from which the puzzles is randomly selected. The
    /// divider is a number of puzzles for the part. The last part can have less the puzzles than
    /// the number of puzzles for the part.
    pub fn select(&self, puzzles: &mut dyn Iterator<Item = SelectorResult<LichessPuzzle>>, writer: &mut dyn Write, divider: u64) -> SelectorResult<()>
    {
        let mut puzzle_writer = LichessPuzzleWriter::from_writer(writer);
        let mut puzzle_count = 0u64;
        let mut i = 0u64;
        for puzzle in puzzles {
            match self.intr_checker.check() {
                Ok(()) => (),
                Err(intr) => return Err(SelectorError::Interruption(intr)),
            }
            let puzzle = puzzle?;
            if puzzle_count % Self::PUZZLE_COUNT_TO_PRINT == 0 {
                let mut writer_g = self.writer.lock().unwrap();
                match self.printer.print(&mut *writer_g, puzzle_count, false) {
                    Ok(()) => (),
                    Err(err) => return Err(SelectorError::Io(err)),
                }
                match writer_g.flush() {
                    Ok(()) => (),
                    Err(err) => return Err(SelectorError::Io(err)),
                }
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
            match self.printer.print(&mut *writer_g, puzzle_count, true) {
                Ok(()) => (),
                Err(err) => return Err(SelectorError::Io(err)),
            }
            match writer_g.flush() {
                Ok(()) => (),
                Err(err) => return Err(SelectorError::Io(err)),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
