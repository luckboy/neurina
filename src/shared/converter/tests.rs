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
use crate::chess::Coord;
use crate::shared::utils::*;
use super::*;

#[test]
fn test_converter_board_to_matrix_col_converts_board_to_matrix_column_for_white()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    let mut elems = vec![0.0f32; Converter::BOARD_ROW_COUNT];
    converter.board_to_matrix_col(&board, elems.as_mut_slice(), 0, 1);
    for squ in 0..64 {
        let coord_idx = coord_to_index(Coord::from_index(squ), Color::White);
        let cell_idx = cell_to_index(board.get(Coord::from_index(coord_idx)), Color::White);
        let i = coord_idx * 13;
        for j in 0..13 {
            if j == cell_idx {
                assert_eq!(1.0, elems[i + j]);
            } else {
                assert_eq!(-1.0, elems[i + j]);
            }
        }
        assert_eq!(1.0, elems[64 * 13 + 0]);
        assert_eq!(1.0, elems[64 * 13 + 1]);
        assert_eq!(-1.0, elems[64 * 13 + 2]);
        assert_eq!(1.0, elems[64 * 13 + 3]);
        assert_eq!(1.0, elems[64 * 13 + 4]);
        assert_eq!(-1.0, elems[64 * 13 + 5]);
        assert_eq!(1.0, elems[64 * 13 + 6 + 0]);
        for i in 0..8 {
            assert_eq!(-1.0, elems[64 * 13 + 6 + i + 1]);
        }
    }
}

#[test]
fn test_converter_board_to_matrix_col_converts_board_to_matrix_column_for_black()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 2").unwrap();
    let mut elems = vec![0.0f32; Converter::BOARD_ROW_COUNT];
    converter.board_to_matrix_col(&board, elems.as_mut_slice(), 0, 1);
    for squ in 0..64 {
        let coord_idx = coord_to_index(Coord::from_index(squ), Color::Black);
        let cell_idx = cell_to_index(board.get(Coord::from_index(coord_idx)), Color::Black);
        let i = coord_idx * 13;
        for j in 0..13 {
            if j == cell_idx {
                assert_eq!(1.0, elems[i + j]);
            } else {
                assert_eq!(-1.0, elems[i + j]);
            }
        }
        assert_eq!(1.0, elems[64 * 13 + 0]);
        assert_eq!(1.0, elems[64 * 13 + 1]);
        assert_eq!(-1.0, elems[64 * 13 + 2]);
        assert_eq!(1.0, elems[64 * 13 + 3]);
        assert_eq!(1.0, elems[64 * 13 + 4]);
        assert_eq!(-1.0, elems[64 * 13 + 5]);
        assert_eq!(1.0, elems[64 * 13 + 6 + 0]);
        for i in 0..8 {
            assert_eq!(-1.0, elems[64 * 13 + 6 + i + 1]);
        }
    }
}

#[test]
fn test_converter_board_to_matrix_col_converts_board_to_matrix_column_for_en_passant()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("r1bqkbnr/ppp1pppp/2n5/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3").unwrap();
    let mut elems = vec![0.0f32; Converter::BOARD_ROW_COUNT];
    converter.board_to_matrix_col(&board, elems.as_mut_slice(), 0, 1);
    for squ in 0..64 {
        let coord_idx = coord_to_index(Coord::from_index(squ), Color::White);
        let cell_idx = cell_to_index(board.get(Coord::from_index(coord_idx)), Color::White);
        let i = coord_idx * 13;
        for j in 0..13 {
            if j == cell_idx {
                assert_eq!(1.0, elems[i + j]);
            } else {
                assert_eq!(-1.0, elems[i + j]);
            }
        }
        assert_eq!(1.0, elems[64 * 13 + 0]);
        assert_eq!(1.0, elems[64 * 13 + 1]);
        assert_eq!(-1.0, elems[64 * 13 + 2]);
        assert_eq!(1.0, elems[64 * 13 + 3]);
        assert_eq!(1.0, elems[64 * 13 + 4]);
        assert_eq!(-1.0, elems[64 * 13 + 5]);
        assert_eq!(-1.0, elems[64 * 13 + 6 + 0]);
        for i in 0..8 {
            if i == 3 {
                assert_eq!(1.0, elems[64 * 13 + 6 + i + 1]);
            } else {
                assert_eq!(-1.0, elems[64 * 13 + 6 + i + 1]);
            }
        }
    }
}

#[test]
fn test_converter_board_to_matrix_col_converts_board_to_matrix_column_for_white_and_castlings()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("r3k3/8/8/8/8/8/8/4K2R w Kq - 0 1").unwrap();
    let mut elems = vec![0.0f32; Converter::BOARD_ROW_COUNT];
    converter.board_to_matrix_col(&board, elems.as_mut_slice(), 0, 1);
    for squ in 0..64 {
        let coord_idx = coord_to_index(Coord::from_index(squ), Color::White);
        let cell_idx = cell_to_index(board.get(Coord::from_index(coord_idx)), Color::White);
        let i = coord_idx * 13;
        for j in 0..13 {
            if j == cell_idx {
                assert_eq!(1.0, elems[i + j]);
            } else {
                assert_eq!(-1.0, elems[i + j]);
            }
        }
        assert_eq!(-1.0, elems[64 * 13 + 0]);
        assert_eq!(1.0, elems[64 * 13 + 1]);
        assert_eq!(-1.0, elems[64 * 13 + 2]);
        assert_eq!(1.0, elems[64 * 13 + 3]);
        assert_eq!(-1.0, elems[64 * 13 + 4]);
        assert_eq!(-1.0, elems[64 * 13 + 5]);
        assert_eq!(1.0, elems[64 * 13 + 6 + 0]);
        for i in 0..8 {
            assert_eq!(-1.0, elems[64 * 13 + 6 + i + 1]);
        }
    }
}

#[test]
fn test_converter_board_to_matrix_col_converts_board_to_matrix_column_for_black_and_castlings()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("r3k3/8/8/8/8/8/8/4K2R b Kq - 0 1").unwrap();
    let mut elems = vec![0.0f32; Converter::BOARD_ROW_COUNT];
    converter.board_to_matrix_col(&board, elems.as_mut_slice(), 0, 1);
    for squ in 0..64 {
        let coord_idx = coord_to_index(Coord::from_index(squ), Color::Black);
        let cell_idx = cell_to_index(board.get(Coord::from_index(coord_idx)), Color::Black);
        let i = coord_idx * 13;
        for j in 0..13 {
            if j == cell_idx {
                assert_eq!(1.0, elems[i + j]);
            } else {
                assert_eq!(-1.0, elems[i + j]);
            }
        }
        assert_eq!(1.0, elems[64 * 13 + 0]);
        assert_eq!(-1.0, elems[64 * 13 + 1]);
        assert_eq!(-1.0, elems[64 * 13 + 2]);
        assert_eq!(-1.0, elems[64 * 13 + 3]);
        assert_eq!(1.0, elems[64 * 13 + 4]);
        assert_eq!(-1.0, elems[64 * 13 + 5]);
        assert_eq!(1.0, elems[64 * 13 + 6 + 0]);
        for i in 0..8 {
            assert_eq!(-1.0, elems[64 * 13 + 6 + i + 1]);
        }
    }
}

#[test]
fn test_converter_board_to_matrix_col_converts_board_to_matrix_column_for_white_and_no_castlings()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("4k3/8/8/7p/7P/8/8/4K3 w - - 0 1").unwrap();
    let mut elems = vec![0.0f32; Converter::BOARD_ROW_COUNT];
    converter.board_to_matrix_col(&board, elems.as_mut_slice(), 0, 1);
    for squ in 0..64 {
        let coord_idx = coord_to_index(Coord::from_index(squ), Color::White);
        let cell_idx = cell_to_index(board.get(Coord::from_index(coord_idx)), Color::White);
        let i = coord_idx * 13;
        for j in 0..13 {
            if j == cell_idx {
                assert_eq!(1.0, elems[i + j]);
            } else {
                assert_eq!(-1.0, elems[i + j]);
            }
        }
        assert_eq!(-1.0, elems[64 * 13 + 0]);
        assert_eq!(-1.0, elems[64 * 13 + 1]);
        assert_eq!(1.0, elems[64 * 13 + 2]);
        assert_eq!(-1.0, elems[64 * 13 + 3]);
        assert_eq!(-1.0, elems[64 * 13 + 4]);
        assert_eq!(1.0, elems[64 * 13 + 5]);
        assert_eq!(1.0, elems[64 * 13 + 6 + 0]);
        for i in 0..8 {
            assert_eq!(-1.0, elems[64 * 13 + 6 + i + 1]);
        }
    }
}

#[test]
fn test_converter_board_to_matrix_col_does_not_set_other_matrix_columns()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    let mut elems = vec![0.0f32; Converter::BOARD_ROW_COUNT * 3];
    converter.board_to_matrix_col(&board, elems.as_mut_slice(), 1, 3);
    for i in 0..Converter::BOARD_ROW_COUNT {
        assert_eq!(0.0, elems[i * 3 + 0]);
        assert_ne!(0.0, elems[i * 3 + 1]);
        assert_eq!(0.0, elems[i * 3 + 2]);
    }
}

#[test]
fn test_converter_move_to_matrix_col_converts_move_to_matrix_column_for_white()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    let mv = Move::from_uci_legal("b1c3", &board).unwrap();
    let mut elems = vec![2.0f32; converter.move_row_count()];
    let move_idx = converter.index_converter().move_to_index(mv, Color::White).unwrap();
    converter.move_to_matrix_col(mv, Color::White, elems.as_mut_slice(), 0, 1);
    for i in 0..converter.move_row_count() {
        if i == move_idx {
            assert_eq!(1.0, elems[i]);
        } else {
            assert_eq!(0.0, elems[i]);
        }
    }
}

#[test]
fn test_converter_move_to_matrix_col_converts_move_to_matrix_column_for_black()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 2").unwrap();
    let mv = Move::from_uci_legal("g8f6", &board).unwrap();
    let mut elems = vec![2.0f32; converter.move_row_count()];
    let move_idx = converter.index_converter().move_to_index(mv, Color::Black).unwrap();
    converter.move_to_matrix_col(mv, Color::Black, elems.as_mut_slice(), 0, 1);
    for i in 0..converter.move_row_count() {
        if i == move_idx {
            assert_eq!(1.0, elems[i]);
        } else {
            assert_eq!(0.0, elems[i]);
        }
    }
}

#[test]
fn test_converter_move_to_matrix_col_does_not_set_other_matrix_columns()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    let mv = Move::from_uci_legal("b1c3", &board).unwrap();
    let mut elems = vec![2.0f32; converter.move_row_count() * 3];
    converter.move_to_matrix_col(mv, Color::White, elems.as_mut_slice(), 1, 3);
    for i in 0..converter.move_row_count() {
        assert_eq!(2.0, elems[i * 3 + 0]);
        assert_ne!(2.0, elems[i * 3 + 1]);
        assert_eq!(2.0, elems[i * 3 + 2]);
    }
}

#[test]
fn test_converter_matrix_col_to_move_converts_matrix_column_to_move_for_white()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    let mv = Move::from_uci_legal("b1c3", &board).unwrap();
    let mut elems = vec![1.0f32; converter.move_row_count()];
    let move_idx = converter.index_converter().move_to_index(mv, Color::White).unwrap();
    elems[move_idx] = 10.0;
    let moves = legal::gen_all(&board);
    match converter.matrix_col_to_move(&moves, Color::White, elems.as_slice(), 0, 1, 0.1) {
        Some(mv2) => assert_eq!(mv, mv2),
        None => assert!(false),
    }
}

#[test]
fn test_converter_matrix_col_to_move_does_not_convert_matrix_column_to_move_for_white()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    let mv = Move::from_uci_legal("b1c3", &board).unwrap();
    let mut elems = vec![9.5f32; converter.move_row_count()];
    let move_idx = converter.index_converter().move_to_index(mv, Color::White).unwrap();
    elems[move_idx] = 10.0;
    let moves = legal::gen_all(&board);
    match converter.matrix_col_to_move(&moves, Color::White, elems.as_slice(), 0, 1, 0.1) {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

#[test]
fn test_converter_matrix_col_to_move_converts_matrix_column_to_move_for_black()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 2").unwrap();
    let mv = Move::from_uci_legal("g8f6", &board).unwrap();
    let mut elems = vec![1.0f32; converter.move_row_count()];
    let move_idx = converter.index_converter().move_to_index(mv, Color::Black).unwrap();
    elems[move_idx] = 10.0;
    let moves = legal::gen_all(&board);
    match converter.matrix_col_to_move(&moves, Color::Black, elems.as_slice(), 0, 1, 0.1) {
        Some(mv2) => assert_eq!(mv, mv2),
        None => assert!(false),
    }
}

#[test]
fn test_converter_matrix_col_to_move_does_not_convert_matrix_column_to_move_for_black()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 2").unwrap();
    let mv = Move::from_uci_legal("g8f6", &board).unwrap();
    let mut elems = vec![9.5f32; converter.move_row_count()];
    let move_idx = converter.index_converter().move_to_index(mv, Color::Black).unwrap();
    elems[move_idx] = 10.0;
    let moves = legal::gen_all(&board);
    match converter.matrix_col_to_move(&moves, Color::Black, elems.as_slice(), 0, 1, 0.1) {
        Some(_) => assert!(false),
        None => assert!(true),
    }
}

#[test]
fn test_converter_matrix_col_to_move_only_checks_one_matrix_column()
{
    let converter = Converter::new(IndexConverter::new());
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
    let mv = Move::from_uci_legal("b1c3", &board).unwrap();
    let mv2 = Move::from_uci_legal("d2d4", &board).unwrap();
    let mv3 = Move::from_uci_legal("g1f3", &board).unwrap();
    let mut elems = vec![1.0f32; converter.move_row_count() * 3];
    let move_idx = converter.index_converter().move_to_index(mv, Color::White).unwrap();
    let move_idx2 = converter.index_converter().move_to_index(mv2, Color::White).unwrap();
    let move_idx3 = converter.index_converter().move_to_index(mv3, Color::White).unwrap();
    elems[move_idx2 * 3 + 0] = 10.0;
    elems[move_idx * 3 + 1] = 10.0;
    elems[move_idx3 * 3 + 2] = 10.0;
    let moves = legal::gen_all(&board);
    match converter.matrix_col_to_move(&moves, Color::White, elems.as_slice(), 1, 3, 0.1) {
        Some(mv4) => assert_eq!(mv, mv4),
        None => assert!(false),
    }
}
