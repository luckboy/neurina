//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::Color;
use crate::chess::Coord;

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
