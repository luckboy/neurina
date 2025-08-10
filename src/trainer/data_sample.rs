//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::Board;
use crate::chess::Move;

/// A structure of data sample.
///
/// The data sample contains a board and moves.
#[derive(Clone, Debug)]
pub struct DataSample
{
    /// A board.
    pub board: Board,
    /// Moves.
    pub moves: Vec<Move>,
}

impl DataSample
{
    /// Creates a data sample.
    pub fn new(board: Board, moves: Vec<Move>) -> Self
    { DataSample { board, moves, } }
}
