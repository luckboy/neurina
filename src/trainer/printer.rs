//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use std::io::Write;
use crate::trainer::print::*;

pub struct Printer;

impl Printer
{
    pub fn new() -> Self
    { Printer }
}

impl Print for Printer
{
    fn print(&self, w: &mut dyn Write, sample_count: u64, computed_minibatch_count: u64, minibatch_count: u64, is_done: bool) -> Result<()>
    {
        if is_done {
            writeln!(w, "computing ({}) ({}/{}) ... done", sample_count, computed_minibatch_count, minibatch_count)?;
        } else {
            write!(w, "computing ({}) ({}/{}) ...\r", sample_count, computed_minibatch_count, minibatch_count)?;
        }
        Ok(())
    }
}
