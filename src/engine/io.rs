//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Stdin;
use std::io::Stdout;
use std::io::Result;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use crate::engine::utils::*;

pub struct StdioLog
{
    stdin: Stdin,
    stdout: Stdout,
    log: Option<Box<dyn Write + Send + Sync>>,
    has_output_prefix: bool,
}

impl StdioLog
{
    pub fn new(log: Option<Box<dyn Write + Send + Sync>>) -> Self
    { StdioLog { stdin: stdin(), stdout: stdout(), log, has_output_prefix: true, } }
    
    pub fn read_line(&mut self, buf: &mut String) -> Result<usize>
    {
        match self.stdin.read_line(buf) {
            Ok(size) => {
                match &mut self.log {
                    Some(log) => {
                        writeln!(log, "input: {}", str_without_nl(buf.as_str()))?;
                    },
                    None => (),
                }
                Ok(size)
            },
            Err(err) => Err(err),
        }
    }
}

impl Write for StdioLog
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>
    {
        let size = self.stdout.write(buf)?;
        match &mut self.log {
            Some(log) => {
                let mut start_idx = 0usize;
                for i in 0..size {
                    if self.has_output_prefix {
                        write!(log, "output: ")?;
                    }
                    if buf[i] == b'\n' {
                        log.write_all(&buf[start_idx..(i + 1)])?;
                        start_idx = i + 1;
                        self.has_output_prefix = true;
                    } else {
                        self.has_output_prefix = false;
                    }
                }
                if start_idx < size {
                    log.write_all(&buf[start_idx..])?;
                }
            },
            None => (),
        }
        Ok(size)
    }
    
    fn flush(&mut self) -> Result<()>
    {
        self.stdout.flush()?;
        match &mut self.log {
            Some(log) => {
                log.flush()?;
            },
            None => (),
        }
        Ok(())
    }
}
