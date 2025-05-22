//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::moves::uci;
use crate::chess::moves::PromotePiece;
use crate::chess::Color;
use crate::chess::Move;
use crate::shared::utils::*;

const MAILBOX: [isize; 120] = [
    -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, -1,
    -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, -1,
    -1, 0o00, 0o01, 0o02, 0o03, 0o04, 0o05, 0o06, 0o07, -1,
    -1, 0o10, 0o11, 0o12, 0o13, 0o14, 0o15, 0o16, 0o17, -1,
    -1, 0o20, 0o21, 0o22, 0o23, 0o24, 0o25, 0o26, 0o27, -1,
    -1, 0o30, 0o31, 0o32, 0o33, 0o34, 0o35, 0o36, 0o37, -1,
    -1, 0o40, 0o41, 0o42, 0o43, 0o44, 0o45, 0o46, 0o47, -1,
    -1, 0o50, 0o51, 0o52, 0o53, 0o54, 0o55, 0o56, 0o57, -1,
    -1, 0o60, 0o61, 0o62, 0o63, 0o64, 0o65, 0o66, 0o67, -1,
    -1, 0o70, 0o71, 0o72, 0o73, 0o74, 0o75, 0o76, 0o77, -1,
    -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, -1,
    -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1,   -1, -1
];

const MAILBOX64: [isize; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28,
    31, 32, 33, 34, 35, 36, 37, 38,
    41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58,
    61, 62, 63, 64, 65, 66, 67, 68,
    71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88,
    91, 92, 93, 94, 95, 96, 97, 98
];

const QUEEN_STEPS120: [isize; 8] = [-11, -10, -9, -1, 1, 9, 10, 11];

const KNIGHT_STEPS120: [isize; 8] = [-21, -19, -12, -8, 8, 12, 19, 21];

#[derive(Clone, Debug)]
pub struct IndexConverter
{
    move_count: usize,
    tab_move_indices: Vec<[[i32; 5]; 64]>,
}

impl IndexConverter
{
    pub fn new() -> Self
    {
        let mut move_count = 0usize;
        let mut tab_move_indices = vec![[[-1; 5]; 64]; 64];
        for from in 0..64 {
            let from120 = MAILBOX64[from];
            for step120 in QUEEN_STEPS120 {
                let mut to120 = from120 + step120;
                while MAILBOX[to120 as usize] != -1 {
                    let to = MAILBOX[to120 as usize];
                    tab_move_indices[from as usize][to as usize][0] = move_count as i32;
                    let to120_from120 = to120 - from120;
                    if (from >> 3) == 1 && (to120_from120 == -11 || to120_from120 == -10 || to120_from120 == -9) {
                        tab_move_indices[from as usize][to as usize][4] = move_count as i32;
                        move_count += 1;
                        for piece in 1..4 {
                            tab_move_indices[from as usize][to as usize][piece] = move_count as i32;
                            move_count += 1;
                        }
                    } else if (from >> 3) == 6 && (to120_from120 == 9 || to120_from120 == 10 || to120_from120 == 11){
                        tab_move_indices[from as usize][to as usize][4] = move_count as i32;
                        move_count += 1;
                        for piece in 1..4 {
                            tab_move_indices[from as usize][to as usize][piece] = move_count as i32;
                            move_count += 1;
                        }
                    } else {
                        move_count += 1;
                    }
                    to120 += step120;
                }
            }
            for step120 in KNIGHT_STEPS120 {
                let to120 = from120 + step120;
                if MAILBOX[to120 as usize] != -1 {
                    let to = MAILBOX[to120 as usize];
                    tab_move_indices[from as usize][to as usize][0] = move_count as i32;
                    move_count += 1;
                }
            }
        }
        IndexConverter { move_count, tab_move_indices, }
    }
    
    pub fn move_count(&self) -> usize
    { self.move_count }
    
    pub fn move_to_index(&self, mv: Move, color: Color) -> Option<usize>
    {
        match mv.uci() {
            uci::Move::Null => None,
            uci::Move::Move { src, dst, promote, } => {
                let src_idx = coord_to_index(src, color);
                let dst_idx = coord_to_index(dst, color);
                let promote_idx = match promote {
                    None => 0,
                    Some(PromotePiece::Knight) => 1,
                    Some(PromotePiece::Bishop) => 2,
                    Some(PromotePiece::Rook) => 3,
                    Some(PromotePiece::Queen) => 4,
                };
                if self.tab_move_indices[src_idx][dst_idx][promote_idx] != -1 {
                    Some(self.tab_move_indices[src_idx][dst_idx][promote_idx] as usize)
                } else {
                    None
                }
            },
        }
    }
}
