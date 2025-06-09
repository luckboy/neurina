//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::path::Path;
use crate::chess::moves::uci;
use crate::chess::moves::PromotePiece;
use crate::chess::Board;
use crate::chess::CastlingSide;
use crate::chess::Color;
use crate::chess::Coord;
use crate::chess::Move;
use crate::chess::Piece;
use crate::engine::fathom;

fn squ_to_fathom_squ(squ: usize) -> usize
{
    let rank = squ >> 3;
    let file = squ & 7;
    ((7 - rank) << 3) | file
}

fn fathom_squ_to_squ(squ: usize) -> usize
{ squ_to_fathom_squ(squ) }

fn board_to_fathom_position(board: &Board) -> fathom::Position
{
    let raw_board = board.raw();
    let mut white = 0u64;
    let mut black = 0u64;
    let mut kings = 0u64;
    let mut queens = 0u64;
    let mut rooks = 0u64;
    let mut bishops = 0u64;
    let mut knights = 0u64;
    let mut pawns = 0u64;
    for squ in 0..64 {
        let cell = raw_board.get(Coord::from_index(squ));
        match cell.color() {
            Some(Color::White) => white |= 1u64 << squ_to_fathom_squ(squ),
            Some(Color::Black) => black |= 1u64 << squ_to_fathom_squ(squ),
            None => (),
        }
        match cell.piece() {
            Some(Piece::Pawn) => pawns |= 1u64 << squ_to_fathom_squ(squ),
            Some(Piece::King) => kings |= 1u64 << squ_to_fathom_squ(squ),
            Some(Piece::Knight) => knights |= 1u64 << squ_to_fathom_squ(squ),
            Some(Piece::Bishop) => bishops |= 1u64 << squ_to_fathom_squ(squ),
            Some(Piece::Rook) => rooks |= 1u64 << squ_to_fathom_squ(squ),
            Some(Piece::Queen) => queens |= 1u64 << squ_to_fathom_squ(squ),
            None => (),
        }
    }
    let rule50 = raw_board.move_counter as u32;
    let mut castling = 0u32;
    if raw_board.castling.has(Color::White, CastlingSide::King) {
        castling |= 1;
    }
    if raw_board.castling.has(Color::White, CastlingSide::Queen) {
        castling |= 2;
    }
    if raw_board.castling.has(Color::Black, CastlingSide::King) {
        castling |= 4;
    }
    if raw_board.castling.has(Color::Black, CastlingSide::Queen) {
        castling |= 8;
    }
    let ep = match raw_board.ep_dest() {
        Some(ep_dest) => squ_to_fathom_squ(ep_dest.index()) as u32,
        None => 0u32,
    };
    let turn = match raw_board.side {
        Color::White => 1u8,
        Color::Black => 0u8,
    };
    fathom::Position {
        white,
        black,
        kings,
        queens,
        rooks,
        bishops,
        knights,
        pawns,
        rule50,
        castling,
        ep,
        turn,
    }
}

fn fathom_move_to_move(board: &Board, fathom_move: fathom::Move) -> Option<Move>
{
    let src = Coord::from_index(fathom_squ_to_squ(u8::from(fathom_move.from) as usize));
    let dst = Coord::from_index(fathom_squ_to_squ(u8::from(fathom_move.to) as usize));
    let promote = match fathom_move.promote {
        fathom::PromotionPiece::None => None,
        fathom::PromotionPiece::Knight => Some(PromotePiece::Knight),
        fathom::PromotionPiece::Bishop => Some(PromotePiece::Bishop),
        fathom::PromotionPiece::Rook => Some(PromotePiece::Rook),
        fathom::PromotionPiece::Queen => Some(PromotePiece::Queen),
    };
    let uci_move = uci::Move::Move { src, dst, promote, };
    match uci_move.into_move(board) {
        Ok(mv) => {
            match mv.validate(board) {
                Ok(()) => Some(mv),
                Err(_) => None,
            }
        },
        Err(_) => None,
    }
}

pub struct Syzygy
{
    fathom: fathom::Fathom,
}

impl Syzygy
{
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Syzygy, fathom::Error>
    { Ok(Syzygy { fathom: fathom::Fathom::new(path)?, }) }

    pub fn reload<P: AsRef<Path>>(self, path: P) -> Result<Syzygy, fathom::Error>
    { Ok(Syzygy { fathom: self.fathom.reload(path)?, }) }
    
    pub fn probe(&mut self, board: &Board) -> Option<Move>
    {
        let (mut root_probe, _) = self.fathom.get_probers();
        let pos = board_to_fathom_position(board);
        match root_probe.probe(&pos) {
            Some(res) => fathom_move_to_move(board, res.best_move),
            None => None,
        }
    }
}
