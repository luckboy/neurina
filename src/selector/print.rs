//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use std::io::Write;

/// A printer trait.
///
/// This trait provides method that prints an information about a selection progress.
pub trait Print
{
    /// Prints the number of puzzles.
    ///
    /// This method prints carriage return as the last character if the completion flag is disabled,
    /// otherwise newline as the character.
    fn print(&self, w: &mut dyn Write, puzzle_count: u64, is_done: bool) -> Result<()>;
}

/// A structure of empty printer.
///
/// The empty printer is dummy that doesn't print anything.
pub struct EmptyPrinter;

impl EmptyPrinter
{
    /// Creates an empty printer.
    pub fn new() -> Self
    { EmptyPrinter }
}

impl Print for EmptyPrinter
{
    fn print(&self, _w: &mut dyn Write, _puzzle_count: u64, _is_done: bool) -> Result<()>
    { Ok(()) }
}
