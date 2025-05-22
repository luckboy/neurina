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
use crate::chess::Color;
use crate::chess::Move;
use crate::matrix::Frontend;
use crate::engine::neural_search::*;
use crate::shared::converter::*;
use crate::shared::intr_check::*;
use crate::shared::matrix_buffer::*;
use crate::shared::net::*;
use crate::shared::Interruption;

pub struct NeuralSearcher<T>
{
    intr_checker: Arc<dyn IntrCheck>,
    converter: Converter,
    matrix_buf: Mutex<MatrixBuffer<Vec<Move>, (Vec<f32>, Vec<Option<(Board, Color)>>)>>,
    network: T,
}

impl<T> NeuralSearcher<T>
{
    pub const MAX_COL_COUNT: usize = 1024;
    
    pub const MOVE_EPS: f32 = 0.01;
    
    pub fn new(intr_checker: Arc<dyn IntrCheck>, converter: Converter, network: T) -> Self
    {
        let matrix_buf = Mutex::new(MatrixBuffer::new(Converter::BOARD_ROW_COUNT, 0, Self::MAX_COL_COUNT, 0, (vec![0.0; converter.move_row_count() * Self::MAX_COL_COUNT], vec![None; Self::MAX_COL_COUNT])));
        NeuralSearcher {
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

impl<T: Net> NeuralSearch for NeuralSearcher<T>
{
    fn intr_checker(&self) -> &Arc<dyn IntrCheck>
    { &self.intr_checker }
    
    fn search(&self, board: &Board, pvs: &mut [Vec<Move>], depth: usize) -> Result<(), Interruption>
    {
        let mut matrix_buf_g = self.matrix_buf.lock().unwrap();
        matrix_buf_g.do_elems_for_slice(pvs, &*self.intr_checker, |pv, elems, _, j, col_count| {
                let mut tmp_board = board.clone();
                for mv in pv {
                    match tmp_board.make_move(*mv) {
                        Ok(tmp_new_board) => tmp_board = tmp_new_board,
                        Err(_) => break,
                    }
                }
                self.converter.board_to_matrix_col(&tmp_board, elems, j, col_count);
        }, |i, _, pair, pvs| {
                let (output_elems, pairs) = pair;
                let col_count = pvs.len();
                for (j, pv) in pvs.iter().enumerate() {
                    let mut tmp_board = board.clone();
                    for mv in pv {
                        match tmp_board.make_move(*mv) {
                            Ok(tmp_new_board) => tmp_board = tmp_new_board,
                            Err(_) => break,
                        }
                    }
                    pairs[j] = Some((tmp_board.clone(), tmp_board.side()));
                }
                self.network.compute(&i, depth, depth, |_| (), |o| {
                        let frontend = Frontend::new().unwrap();
                        let mut is_transposed = false;
                        frontend.get_elems_and_transpose_flag(&o, &mut output_elems[0..(self.converter.move_row_count() * col_count)], &mut is_transposed).unwrap();
                        for (j, pv) in pvs.iter_mut().enumerate() {
                            match &pairs[j] {
                                Some((tmp_board, color)) => {
                                    let moves = legal::gen_all(&tmp_board);
                                    match self.converter.matrix_col_to_move(&moves, *color, output_elems.as_slice(), j, col_count, Self::MOVE_EPS) {
                                        Some(mv) => {
                                            match tmp_board.make_move(mv) {
                                                Ok(tmp_new_board) => {
                                                    pv.push(mv);
                                                    pairs[j] = Some((tmp_new_board, *color));
                                                },
                                                Err(_) => pairs[j] = None,
                                            }
                                        },
                                        None => pairs[j] = None,
                                    }
                                },
                                None => (),
                            }
                        }
                });
        })
    }
}
