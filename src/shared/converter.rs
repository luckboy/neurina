//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::Board;
use crate::chess::CastlingSide;
use crate::chess::Color;
use crate::chess::Coord;
use crate::chess::Move;
use crate::chess::MoveList;
use crate::shared::index_converter::*;
use crate::shared::utils::*;

#[derive(Clone, Debug)]
pub struct Converter
{
    index_converter: IndexConverter,
}

impl Converter
{
    pub const BOARD_ROW_COUNT: usize = 64 * 13 + 6 + 9;

    pub fn new(index_converter: IndexConverter) -> Self
    { Converter { index_converter, } }

    pub fn index_converter(&self) -> &IndexConverter
    { &self.index_converter }
    
    pub fn move_row_count(&self) -> usize
    { self.index_converter.move_count() }
    
    pub fn board_to_matrix_col(&self, board: &Board, elems: &mut [f32], col: usize, col_count: usize)
    {
        for i in 0..Self::BOARD_ROW_COUNT {
            elems[col_count * i + col] = -1.0;
        }
        let side = board.side();
        let raw_board = board.raw();
        let mut offset = 0usize;
        for squ in 0..64 {
            let coord_idx = coord_to_index(Coord::from_index(squ), side);
            let cell_idx = cell_to_index(raw_board.get(Coord::from_index(coord_idx)), side);
            elems[col_count * (offset + coord_idx * 13 + cell_idx) + col] = 1.0;
        }
        offset += 64 * 13;
        let wq_castling = raw_board.castling.has(side, CastlingSide::Queen);
        let wk_castling = raw_board.castling.has(side, CastlingSide::King);
        let we_castling = !(wq_castling | wk_castling);
        let bq_castling = raw_board.castling.has(side.inv(), CastlingSide::Queen);
        let bk_castling = raw_board.castling.has(side.inv(), CastlingSide::King);
        let be_castling = !(bq_castling | bk_castling);
        if wq_castling {
            elems[col_count * (offset + 0) + col] = 1.0;
        }
        if wk_castling {
            elems[col_count * (offset + 1) + col] = 1.0;
        }
        if we_castling {
            elems[col_count * (offset + 2) + col] = 1.0;
        }
        if bq_castling {
            elems[col_count * (offset + 3) + col] = 1.0;
        }
        if bk_castling {
            elems[col_count * (offset + 4) + col] = 1.0;
        }
        if be_castling {
            elems[col_count * (offset + 5) + col] = 1.0;
        }
        offset += 6;
        match raw_board.ep_dest() {
            Some(ep_dest) => elems[col_count * (offset + ep_dest.file().index() + 1) + col] = 1.0,
            None => elems[col_count * (offset + 0) + col] = 1.0,
        }
    }
    
    pub fn move_to_matrix_col(&self, mv: Move, color: Color, elems: &mut [f32], col: usize, col_count: usize)
    {
        for i in 0..self.move_row_count() {
            elems[col_count * i + col] = 0.0;
        }
        match self.index_converter.move_to_index(mv, color) {
            Some(idx) => elems[col_count * idx + col] = 1.0,
            None => (),
        }
    }

    pub fn matrix_col_to_move(&self, moves: &MoveList, color: Color, elems: &[f32], col: usize, col_count: usize, eps: f32) -> Option<Move>
    {
        let mut best_move_score = -f32::INFINITY;
        let mut worst_move_score = f32::INFINITY;
        let mut best_move: Option<Move> = None;
        let mut move_count = 0usize;
        for mv in moves {
            match self.index_converter.move_to_index(*mv, color) {
                Some(idx) => {
                    if elems[col_count * idx + col] > best_move_score {
                        best_move = Some(*mv);
                        best_move_score = elems[col_count * idx + col];
                    }
                    if elems[col_count * idx + col] < worst_move_score {
                        worst_move_score = elems[col_count * idx + col];
                    }
                    move_count += 1;
                },
                None => (),
            }
        }
        match best_move {
            Some(best_move) => {
                if move_count <= 1 || (best_move_score - worst_move_score).abs() > best_move_score.abs() * eps {
                    Some(best_move)
                } else {
                    None
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;
