//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use csv::DeserializeRecordsIter;
use csv::Reader;
use csv::Writer;
use crate::selector::SelectorError;
use crate::selector::SelectorResult;
use crate::shared::lichess_puzzle::*;
use crate::shared::private::*;

pub struct LichessPuzzleReader<R>
{
    reader: Reader<R>,
}

impl LichessPuzzleReader<File>
{
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match Reader::from_path(path) {
            Ok(reader) => Ok(LichessPuzzleReader { reader, }),
            Err(err) => Err(csv_error_to_io_error(err)),
        }
    }
}

impl<R: Read> LichessPuzzleReader<R>
{
    pub fn from_reader(r: R) -> Self
    { LichessPuzzleReader { reader: Reader::from_reader(r), } }
    
    pub fn puzzles(&mut self, max_count: Option<u64>) -> LichessPuzzles<'_, R>
    { LichessPuzzles { iter: self.reader.deserialize(), count: 0, max_count, } }
}

pub struct LichessPuzzles<'a, R>
{
    iter: DeserializeRecordsIter<'a, R, LichessPuzzle>,
    count: u64,
    max_count: Option<u64>,
}

impl<'a, R: Read> Iterator for LichessPuzzles<'a, R>
{
    type Item = SelectorResult<LichessPuzzle>;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        let can_read = match self.max_count {
            Some(max_count) if self.count < max_count => true,
            Some(_) => false,
            None => true,
        };
        if can_read {
            match self.iter.next() {
                Some(Ok(puzzle)) => Some(Ok(puzzle)),
                Some(Err(err)) => Some(Err(SelectorError::Io(csv_error_to_io_error(err)))),
                None => None,
            }
        } else {
            None
        }
    }
}

pub struct LichessPuzzleWriter<W: Write>
{
    writer: Writer<W>,
}

impl LichessPuzzleWriter<File>
{
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self>
    {
        match Writer::from_path(path) {
            Ok(writer) => Ok(LichessPuzzleWriter { writer, }),
            Err(err) => Err(csv_error_to_io_error(err)),
        }
    }
}

impl<W: Write> LichessPuzzleWriter<W>
{
    pub fn from_writer(w: W) -> Self
    { LichessPuzzleWriter { writer: Writer::from_writer(w), } }
    
    pub fn write_puzzle(&mut self, puzzle: &LichessPuzzle) -> SelectorResult<()>
    {
        match self.writer.serialize(puzzle) {
            Ok(()) => Ok(()),
            Err(err) => Err(SelectorError::Io(csv_error_to_io_error(err))),
        }
    }
}
