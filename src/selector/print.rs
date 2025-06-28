//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use std::io::Write;

pub trait Print
{
    fn print(&self, w: &mut dyn Write, puzzle_count: u64, is_done: bool) -> Result<()>;
}

pub struct EmptyPrinter;

impl EmptyPrinter
{
    pub fn new() -> Self
    { EmptyPrinter }
}

impl Print for EmptyPrinter
{
    fn print(&self, _w: &mut dyn Write, _puzzle_count: u64, _is_done: bool) -> Result<()>
    { Ok(()) }
}
