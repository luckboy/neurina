//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::path::Path;
use csv::DeserializeRecordsIter;
use csv::Reader;
use crate::chess::Board;
use crate::chess::Move;
use crate::serde::Deserialize;
use crate::trainer::data_sample::*;
use crate::trainer::TrainerError;
use crate::trainer::TrainerResult;

fn csv_error_to_io_error(err: csv::Error) -> Error
{
    if err.is_io_error() {
        match err.into_kind() {
            csv::ErrorKind::Io(err) => err,
            _ => Error::new(ErrorKind::InvalidData, format!("csv error: unknown error")),
        }
    } else {
        Error::new(ErrorKind::InvalidData, format!("csv error: {}", err))
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize)]
struct LichessPuzzle
{
    PuzzleId: String,
    FEN: String,
    Moves: String,
    Rating: String,
    RatingDeviation: String,
    Popularity: String,
    NbPlays: String,
    Themes: String,
    GameUrl: String,
    OpeningTags: String,
}

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
    type Item = TrainerResult<Option<DataSample>>;
    
    fn next(&mut self) -> Option<Self::Item>
    {
        let can_read = match self.max_count {
            Some(max_count) if self.count < max_count => true,
            Some(_) => false,
            None => true,
        };
        if can_read {
            match self.iter.next() {
                Some(puzzle) => {
                    match puzzle {
                        Ok(puzzle) => {
                            self.count += 1;
                            let mut board = match Board::from_fen(puzzle.FEN.as_str()) {
                                Ok(tmp_board) => tmp_board,
                                Err(_) => return Some(Ok(None)),
                            };
                            let mut ss = puzzle.Moves.split_whitespace();
                            match ss.next() {
                                Some(s) => {
                                    board = match Move::from_uci_legal(s, &board) {
                                        Ok(mv) => {
                                            match board.make_move(mv) {
                                                Ok(tmp_board) => tmp_board,
                                                Err(_) => return Some(Ok(None)),
                                            }
                                        },
                                        Err(_) => return Some(Ok(None)),
                                    };
                                    let mut tmp_board = board.clone();
                                    let mut moves: Vec<Move> = Vec::new();
                                    for s2 in ss {
                                        match Move::from_uci_legal(s2, &tmp_board) {
                                            Ok(mv) => {
                                                match tmp_board.make_move(mv) {
                                                    Ok(tmp_new_board) => {
                                                        tmp_board = tmp_new_board;
                                                        moves.push(mv);
                                                    },
                                                    Err(_) => return Some(Ok(None)),
                                                }
                                            },
                                            Err(_) => return Some(Ok(None)),
                                        }
                                    }
                                    Some(Ok(Some(DataSample::new(board, moves))))
                                },
                                None => Some(Ok(None)),
                            }
                        },
                        Err(err) => Some(Err(TrainerError::Io(csv_error_to_io_error(err)))),
                    }
                },
                None => None,
            }
        } else {
            None
        }
    }
}
