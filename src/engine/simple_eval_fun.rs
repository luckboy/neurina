//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::Board;
use crate::chess::Color;
use crate::chess::Coord;
use crate::chess::Piece;
use crate::engine::eval::*;
use crate::shared::utils::*;

const PIECE_SQUARES: [[i32; 64]; 6] = [
    // Pawn
    [
          0,   0,   0,   0,   0,   0,   0,   0,
         15,  17,  19,  21,  21,  19,  17,  15,
         12,  14,  16,  18,  18,  16,  14,  12,
          9,  11,  13,  15,  15,  13,  11,   9,
          6,   8,  10,  12,  12,  10,   8,   6,
          3,   5,   7,   9,   9,   7,   5,   3,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0
    ],
    // King
    [
        -40, -40, -40, -40, -40, -40, -40, -40,
        -40, -40, -40, -40, -40, -40, -40, -40,
        -40, -40, -40, -40, -40, -40, -40, -40,
        -40, -40, -40, -40, -40, -40, -40, -40,
        -40, -40, -40, -40, -40, -40, -40, -40,
        -40, -40, -40, -40, -40, -40, -40, -40,
        -20, -20, -20, -20, -20, -20, -20, -20,
          0,   0,  20,   0,   0,   0,  20,   0
    ],
    // Knight
    [
        -10,  -5,   0,   3,   3,   0,  -5, -10,
         -5,   0,   3,   6,   6,   3,   0,  -5,
          0,   3,   6,   9,   9,   6,   3,   0,
          3,   6,   9,  12,  12,   9,   6,   3,
          3,   6,   9,  12,  12,   9,   6,   3,
          0,   3,   6,   9,   9,   6,   3,   0,
         -5,   0,   3,   6,   6,   3,   0,  -5,
        -10,  -5,   0,   3,   3,   0,  -5, -10
    ],
    // Bishop
    [
        -10,  -5,   0,   3,   3,   0,  -5, -10,
         -5,   0,   3,   6,   6,   3,   0,  -5,
          0,   3,   6,   9,   9,   6,   3,   0,
          3,   6,   9,  12,  12,   9,   6,   3,
          3,   6,   9,  12,  12,   9,   6,   3,
          0,   3,   6,   9,   9,   6,   3,   0,
         -5,   0,   3,   6,   6,   3,   0,  -5,
        -10,  -5,   0,   3,   3,   0,  -5, -10
    ],
    // Rook
    [
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0
    ],
    // Queen
    [
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0
    ]
];

const ENDGAME_MATERIAL_VALUE: i32 = 3000;

const ENDGAME_KING_SQUARES: [i32; 64] = [
      0,  10,  20,  30,  30,  20,  10,   0,
     10,  20,  30,  40,  40,  30,  20,  10,
     20,  30,  40,  50,  50,  40,  30,  20,
     30,  40,  50,  60,  60,  50,  40,  30,
     30,  40,  50,  60,  60,  50,  40,  30,
     20,  30,  40,  50,  50,  40,  30,  20,
     10,  20,  30,  40,  40,  30,  20,  10,
      0,  10,  20,  30,  30,  20,  10,   0
];


const PIECE_MATERIAL_VALUES: [i32; 6] = [
    100,    // Pawn
    0,      // King
    300,    // Knight
    300,    // Bishop
    600,    // Rook
    1000    // Queen
];

/// A structure of simple evaluation function.
///
/// The simple evaluation function uses material values and piece/square tables to a board
/// evaluation.
#[derive(Copy, Clone, Debug)]
pub struct SimpleEvalFun;

impl SimpleEvalFun
{
    /// Creates a simple evaluation function.
    pub fn new() -> Self
    { SimpleEvalFun }
}

impl Eval for SimpleEvalFun
{
    fn evaluate(&self, board: &Board) -> i32
    {
        let mut white_material_value = 0;
        let mut black_material_value = 0;
        for squ in 0..64 {
            let cell = board.get(Coord::from_index(squ));
            match (cell.color(), cell.piece()) {
                (Some(Color::White), Some(piece)) => white_material_value += PIECE_MATERIAL_VALUES[piece.index()],
                (Some(Color::Black), Some(piece)) => black_material_value += PIECE_MATERIAL_VALUES[piece.index()],
                _ => (),
            }
        }
        let mut value = white_material_value - black_material_value;
        for squ in 0..64 {
            let cell = board.get(Coord::from_index(squ));
            match (cell.color(), cell.piece()) {
                (Some(color), Some(piece)) => {
                    let psqu_squ = coord_to_index(Coord::from_index(squ), color);
                    let psqu_value = if piece == Piece::King && white_material_value + black_material_value <= ENDGAME_MATERIAL_VALUE {
                        ENDGAME_KING_SQUARES[psqu_squ]
                    } else {
                        PIECE_SQUARES[piece.index()][psqu_squ]
                    };
                    match color {
                        Color::White => value += psqu_value,
                        Color::Black => value -= psqu_value,
                    }
                },
                _ => (),
            }
        }
        match board.side() {
            Color::White => value,
            Color::Black => -value,
        }
    }
}
