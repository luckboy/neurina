//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use std::sync::Mutex;
use crate::chess::movegen::legal;
use crate::chess::Board;
use crate::chess::Move;
use crate::matrix::Frontend;
use crate::matrix::Matrix;
use crate::engine::neural_search::*;
use crate::shared::converter::*;
use crate::shared::intr_check::*;
use crate::shared::matrix_buffer::*;
use crate::shared::net::*;
use crate::shared::Interruption;

pub struct OneNeuralSearcher<T>
{
    intr_checker: Arc<dyn IntrCheck + Send + Sync>,
    converter: Converter,
    matrix_buf: Mutex<MatrixBuffer<(Vec<f32>, Vec<f32>, Vec<Board>)>>,
    network: T,
}

impl<T> OneNeuralSearcher<T>
{
    pub const MAX_COL_COUNT: usize = 1024;
    
    pub const MOVE_EPS: f32 = 0.01;
    
    pub fn new(intr_checker: Arc<dyn IntrCheck + Send + Sync>, converter: Converter, network: T) -> Self
    {
        let matrix_buf = Mutex::new(MatrixBuffer::new(Converter::BOARD_ROW_COUNT, 0, Self::MAX_COL_COUNT, 0, (vec![0.0; Converter::BOARD_ROW_COUNT * Self::MAX_COL_COUNT], vec![0.0; converter.move_row_count() * Self::MAX_COL_COUNT], vec![Board::initial(); Self::MAX_COL_COUNT])));
        OneNeuralSearcher {
            intr_checker,
            converter,
            matrix_buf,
            network,
        }
    }
    
    pub fn converter(&self) -> &Converter
    { &self.converter }

    pub fn network(&self) -> &T
    { &self.network }
}

impl<T: Net> NeuralSearch for OneNeuralSearcher<T>
{
    fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync + 'static>
    { &self.intr_checker }
    
    fn search(&self, board: &Board, pvs: &mut [Vec<Move>], depth: usize) -> Result<(), Interruption>
    {
        let mut matrix_buf_g = self.matrix_buf.lock().unwrap();
        matrix_buf_g.do_elems(pvs, 0, &*self.intr_checker, |pv, elems, _, j, col_count| {
                let mut tmp_board = board.clone();
                for mv in pv {
                    match tmp_board.make_move(*mv) {
                        Ok(tmp_new_board) => tmp_board = tmp_new_board,
                        Err(_) => break,
                    }
                }
                self.converter.board_to_matrix_col(&tmp_board, elems, j, col_count);
        }, |i, _, tuple, pvs| {
                let (input_elems, output_elems, boards) = tuple;
                let col_count = pvs.len();
                for (j, pv) in pvs.iter().enumerate() {
                    let mut tmp_board = board.clone();
                    for mv in pv {
                        match tmp_board.make_move(*mv) {
                            Ok(tmp_new_board) => tmp_board = tmp_new_board,
                            Err(_) => break,
                        }
                    }
                    boards[j] = tmp_board.clone();
                }
                let mut tmp_i = i.clone();
                let mut is_first = true;
                for _ in 0..depth {
                    if !is_first {
                        for j in 0..col_count {
                            self.converter.board_to_matrix_col(&boards[j], &mut input_elems[0..(Converter::BOARD_ROW_COUNT * col_count)], j, col_count);
                        }
                        tmp_i = Matrix::new_with_elems(Converter::BOARD_ROW_COUNT, col_count, &input_elems[0..(Converter::BOARD_ROW_COUNT * col_count)]);
                    }
                    self.network.compute(&tmp_i, 1, 1, |_| self.intr_checker.check(), |o| {
                            self.intr_checker.check()?;
                            let frontend = Frontend::new().unwrap();
                            let mut is_transposed = false;
                            frontend.get_elems_and_transpose_flag(&o, &mut output_elems[0..(self.converter.move_row_count() * col_count)], &mut is_transposed).unwrap();
                            for (j, pv) in pvs.iter_mut().enumerate() {
                                let moves = legal::gen_all(&boards[j]);
                                match self.converter.matrix_col_to_move(&moves, boards[j].side(), output_elems.as_slice(), j, col_count, Self::MOVE_EPS) {
                                    Some(mv) => {
                                        match boards[j].make_move(mv) {
                                            Ok(tmp_new_board) => {
                                                pv.push(mv);
                                                boards[j] = tmp_new_board;
                                            },
                                            Err(_) => (),
                                        }
                                    },
                                    None => (),
                                }
                            }
                            Ok(())
                    })?;
                    is_first = false;
                }
                Ok(())
        })
    }
}
