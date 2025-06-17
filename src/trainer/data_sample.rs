//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::Board;
use crate::chess::Move;

#[derive(Clone, Debug)]
pub struct DataSample
{
    pub board: Board,
    pub moves: Vec<Move>,
}

impl DataSample
{
    pub fn new(board: Board, moves: Vec<Move>) -> Self
    { DataSample { board, moves, } }
}
