//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use std::marker::PhantomData;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use crate::chess::Board;
use crate::matrix::Frontend;
use crate::matrix::Matrix;
use crate::shared::converter::*;
use crate::shared::intr_check::*;
use crate::shared::io::*;
use crate::shared::matrix_buffer::*;
use crate::shared::net::*;
use crate::trainer::algorithm::*;
use crate::trainer::data_sample::*;
use crate::trainer::gradient_add::*;
use crate::trainer::gradient_add_create::*;
use crate::trainer::gradient_pair::*;
use crate::trainer::io::*;
use crate::trainer::net_create::*;
use crate::trainer::TrainerError;
use crate::trainer::TrainerResult;

#[derive(Copy, Clone, Debug)]
pub struct OneGradientAdderFactory<T, NL, NF>
{
    net_loader: NL,
    xavier_net_factory: NF,
    _unused: PhantomData<T>,
}

impl<T, NL, NF> OneGradientAdderFactory<T, NL, NF>
{
    pub fn new(net_loader: NL, xavier_net_factory: NF) -> Self
    {
        OneGradientAdderFactory {
            net_loader,
            xavier_net_factory,
            _unused: PhantomData::<T>,
        }
    }
}

impl<T, NL: Load<T>, NF: NetCreate<T>> GradientAddCreate<OneGradientAdder<T>> for OneGradientAdderFactory<T, NL, NF>
{
    fn create(&self, intr_checker: Arc<dyn IntrCheck + Send + Sync>, converter: Converter) -> Result<OneGradientAdder<T>>
    {
        let network = load_or_else(&self.net_loader, NETWORK_NAME, || self.xavier_net_factory.create(Converter::BOARD_ROW_COUNT, converter.move_row_count()))?;
        Ok(OneGradientAdder::new(intr_checker, converter, network))
    }
}

pub struct OneGradientAdder<T>
{
    intr_checker: Arc<dyn IntrCheck + Send + Sync>,
    converter: Converter,
    matrix_buf: Mutex<MatrixBuffer<(Vec<f32>, Vec<f32>, Vec<f32>, Vec<Board>)>>,
    network: Mutex<T>,
    gradient: Mutex<Option<T>>,
    all_output_count: AtomicU64,
}

impl<T> OneGradientAdder<T>
{
    pub const MAX_COL_COUNT: usize = 1024;
    
    pub fn new(intr_checker: Arc<dyn IntrCheck + Send + Sync>, converter: Converter, network: T) -> Self
    { Self::new_with_max_col_count(intr_checker, converter, network, Self::MAX_COL_COUNT) }
    
    pub fn new_with_max_col_count(intr_checker: Arc<dyn IntrCheck + Send + Sync>, converter: Converter, network: T, max_col_count: usize) -> Self
    {
        let matrix_buf = Mutex::new(MatrixBuffer::new(Converter::BOARD_ROW_COUNT, converter.move_row_count(), max_col_count, 0, (vec![0.0; Converter::BOARD_ROW_COUNT * max_col_count], vec![0.0; converter.move_row_count() * max_col_count], vec![0.0; converter.move_row_count() * max_col_count], vec![Board::initial(); max_col_count])));
        OneGradientAdder {
            intr_checker,
            converter,
            matrix_buf,
            network: Mutex::new(network),
            gradient: Mutex::new(None),
            all_output_count: AtomicU64::new(0),
        }
    }
    
    pub fn converter(&self) -> &Converter
    { &self.converter }

    pub fn network(&self) -> &Mutex<T>
    { &self.network }

    pub fn gradient(&self) -> &Mutex<Option<T>>
    { &self.gradient }
}

impl<T: Net> GradientAdd for OneGradientAdder<T>
{
    fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>
    { &self.intr_checker }
    
    fn samples_are_full(&self, sample_count: usize) -> bool
    {
        let matrix_buf_g = self.matrix_buf.lock().unwrap();
        matrix_buf_g.elems_are_full(sample_count)
    }

    fn start(&self)
    {
        let mut gradient_g = self.gradient.lock().unwrap();
        *gradient_g = None;
        self.all_output_count.store(0, Ordering::SeqCst);
    }
    
    fn compute(&self, samples: &mut [DataSample], move_count: usize, are_gradients: bool) -> TrainerResult<(u64, u64)>
    {
        let mut passed_output_count = 0u64;
        let mut all_output_count = 0u64;
        let network_g = self.network.lock().unwrap();
        let mut gradient_g = self.gradient.lock().unwrap();
        let mut matrix_buf_g = self.matrix_buf.lock().unwrap();
        if move_count > matrix_buf_g.max_output_count() {
            matrix_buf_g.resize_output_bufs(move_count);
        }
        let res = matrix_buf_g.do_elems(samples, move_count, &*self.intr_checker, |sample, input_elems, output_elems, j, col_count| {
                self.converter.board_to_matrix_col(&sample.board, input_elems, j, col_count);
                let mut color = sample.board.side();
                for k in 0..output_elems.len() {
                    self.converter.move_to_matrix_col(sample.moves[k], color, output_elems[k].as_mut_slice(), j, col_count);
                    color = color.inv();
                }
        }, |i, ys, tuple, samples| {
            let col_count = samples.len();
            let (input_elems, o_elems, y_elems, boards) = tuple;
            for (sample, board) in samples.iter().zip(boards.iter_mut()) {
                *board = sample.board.clone();
            }
            let mut tmp_i = i.clone();
            let mut is_first = true;
            for (ply, y) in ys.iter().enumerate() {
                let mut hs: Vec<Matrix> = Vec::new();
                let mut os: Vec<Matrix> = Vec::new();
                if !is_first {
                    for (j, board) in boards.iter().enumerate() {
                        self.converter.board_to_matrix_col(board, &mut input_elems[0..(Converter::BOARD_ROW_COUNT * col_count)], j, col_count);
                    }
                    tmp_i = Matrix::new_with_elems(Converter::BOARD_ROW_COUNT, col_count, &input_elems[0..(Converter::BOARD_ROW_COUNT * col_count)]);
                }
                network_g.compute(&tmp_i, 1, 1, |h| {
                        if are_gradients {
                            hs.push(h);
                        }
                        Ok(())
                }, |o| {
                    os.push(o);
                    Ok(())
                })?;
                let frontend = Frontend::new().unwrap();
                let mut is_transposed = false;
                frontend.get_elems_and_transpose_flag(&os[0], &mut o_elems[0..(self.converter.move_row_count() * col_count)], &mut is_transposed).unwrap();
                frontend.get_elems_and_transpose_flag(&y, &mut y_elems[0..(self.converter.move_row_count() * col_count)], &mut is_transposed).unwrap();
                let mut best_move_idxs = vec![None::<usize>; col_count];
                for j in 0..self.converter.move_row_count() {
                    for k in 0..col_count {
                        match best_move_idxs[k] {
                            Some(best_move_idx) => {
                                if o_elems[col_count * j + k] > o_elems[col_count * best_move_idx + k] {
                                    best_move_idxs[k] = Some(j);
                                }
                            },
                            None => best_move_idxs[k] = Some(j),
                        }
                    }
                }
                for k in 0..col_count {
                    match best_move_idxs[k] {
                        Some(best_move_idx) => {
                            if y_elems[col_count * best_move_idx + k] > 0.0 {
                                passed_output_count += 1;
                            }
                        }
                        None => (),
                    }
                    all_output_count += 1;
                }
                if are_gradients {
                    let one_elems = vec![0.0f32; col_count];
                    let one = Matrix::new_with_elems(col_count, 1, one_elems.as_slice());
                    let dj_dnet = network_g.backpropagate(&tmp_i, hs.as_slice(), os.as_slice(), &ys[ply..(ply + 1)], &one);
                    match &mut *gradient_g {
                        Some(gradient) => gradient.op_assign(&dj_dnet, |a, b| *a += b),
                        None => *gradient_g = Some(dj_dnet),
                    }
                }
                for (sample, board) in samples.iter().zip(boards.iter_mut()) {
                    match board.make_move(sample.moves[ply]) {
                        Ok(new_board) => *board = new_board,
                        Err(_) => (),
                    }
                }
                is_first = false;
            }
            Ok(())
        });
        match res {
            Ok(()) => {
                self.all_output_count.fetch_add(all_output_count, Ordering::SeqCst);
                Ok((passed_output_count, all_output_count))
            },
            Err(intr) => Err(TrainerError::Interruption(intr)),
        }
    }
    
    fn divide(&self) -> TrainerResult<()>
    {
        let mut gradient_g = self.gradient.lock().unwrap();
        match &mut *gradient_g {
            Some(gradient) => {
                *gradient = gradient.fun(|a| a / (self.all_output_count.load(Ordering::SeqCst) as f32));
                Ok(())
            },
            None => Err(TrainerError::NoGradient),
        }
    }
}

impl<T> GradientPair<T> for OneGradientAdder<T>
{
    fn network_in<U, F>(&self, f: F) -> U
        where F: FnOnce(&mut T) -> U
    {
        let mut network_g = self.network.lock().unwrap();
        f(&mut *network_g)
    }
    
    fn gradient_in<U, F>(&self, f: F) -> TrainerResult<U>
        where F: FnOnce(&T) -> U
    {
        let gradient_g = self.gradient.lock().unwrap();
        match &*gradient_g {
            Some(gradient) => Ok(f(gradient)),
            None => Err(TrainerError::NoGradient),
        }
    }

    fn network_and_gradient_in<U, F>(&self, f: F) -> TrainerResult<U>
        where F: FnOnce(&mut T, &T) -> U
    {
        let mut network_g = self.network.lock().unwrap();
        let gradient_g = self.gradient.lock().unwrap();
        match &*gradient_g {
            Some(gradient) => Ok(f(&mut *network_g, gradient)),
            None => Err(TrainerError::NoGradient),
        }
    }
}
