//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use std::io::Write;
use crate::selector::print::*;

pub struct Printer;

impl Printer
{
    pub fn new() -> Self
    { Printer }
}

impl Print for Printer
{
    fn print(&self, w: &mut dyn Write, puzzle_count: u64, is_done: bool) -> Result<()>
    {
        if is_done {
            writeln!(w, "selecting ({}) ... done", puzzle_count)?;
        } else {
            write!(w, "selecting ({}) ...\r", puzzle_count)?;
        }
        Ok(())
    }
}
