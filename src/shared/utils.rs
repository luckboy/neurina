//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::Cell;
use crate::chess::Color;
use crate::chess::Coord;

/// Converts the square coordinate to an index for a matrix.
pub fn coord_to_index(coord: Coord, color: Color) -> usize
{
    let idx = coord.index();
    match color {
        Color::White => idx,
        Color::Black => {
            let rank_idx = idx >> 3;
            let file_idx = idx & 7;
            ((7 - rank_idx) << 3) | file_idx
        },
    }
}

/// Converts the board cell to an index for a matrix.
pub fn cell_to_index(cell: Cell, color: Color) -> usize
{
    let tmp_cell = match color {
        Color::White => cell,
        Color::Black => {
            match (cell.color(), cell.piece()) {
                (Some(piece_color), Some(piece)) => Cell::from_parts(piece_color.inv(), piece),
                (_, _) => cell,
            }
        },
    };
    tmp_cell.index()
}
