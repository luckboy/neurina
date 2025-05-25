//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::movegen::legal;
use crate::chess::Board;
use crate::chess::Color;
use super::*;

#[test]
fn test_index_converter_new_creates_index_converter_with_number_of_moves()
{
    let index_converter = IndexConverter::new();
    assert_eq!(1924, index_converter.move_count());
}

#[test]
fn test_index_converter_move_to_index_converts_moves_to_indices_for_white_side_and_white_color()
{
    let index_converter = IndexConverter::new();
    let board = Board::from_fen("r1b1k3/1P6/8/8/5N2/8/1Q6/4K3 w - - 0 1").unwrap();
    let moves = legal::gen_all(&board);
    for mv in &moves {
        assert_ne!(None, index_converter.move_to_index(*mv, Color::White));
    }
}

#[test]
fn test_index_converter_move_to_index_converts_moves_to_indices_for_white_side_and_black_color()
{
    let index_converter = IndexConverter::new();
    let board = Board::from_fen("r1b1k3/1P6/8/8/5N2/8/1Q6/4K3 w - - 0 1").unwrap();
    let moves = legal::gen_all(&board);
    for mv in &moves {
        assert_ne!(None, index_converter.move_to_index(*mv, Color::Black));
    }
}

#[test]
fn test_index_converter_move_to_index_converts_moves_to_indices_for_black_side_and_white_color()
{
    let index_converter = IndexConverter::new();
    let board = Board::from_fen("4k3/1q6/8/5n2/8/8/1p6/R1B1K3 b - - 0 1").unwrap();
    let moves = legal::gen_all(&board);
    for mv in &moves {
        assert_ne!(None, index_converter.move_to_index(*mv, Color::White));
    }
}

#[test]
fn test_index_converter_move_to_index_converts_moves_to_indices_for_black_side_and_black_color()
{
    let index_converter = IndexConverter::new();
    let board = Board::from_fen("4k3/1q6/8/5n2/8/8/1p6/R1B1K3 b - - 0 1").unwrap();
    let moves = legal::gen_all(&board);
    for mv in &moves {
        assert_ne!(None, index_converter.move_to_index(*mv, Color::Black));
    }
}
