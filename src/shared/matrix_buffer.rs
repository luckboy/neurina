//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cmp::min;
use crate::matrix::Matrix;
use crate::shared::intr_check::*;
use crate::shared::Interruption;

pub struct MatrixBuffer<T>
{
    elems: Vec<T>,
    input_row_count: usize,
    output_row_count: usize,
    max_col_count: usize,
    input_buf: Vec<f32>,
    output_bufs: Vec<Vec<f32>>,
}

impl<T> MatrixBuffer<T>
{
    pub fn new(input_row_count: usize, output_row_count: usize, max_col_count: usize, output_count: usize) -> Self
    {
        MatrixBuffer {
            elems: Vec::new(),
            input_row_count,
            output_row_count,
            max_col_count,
            input_buf: vec![0.0f32; input_row_count * max_col_count],
            output_bufs: vec![vec![0.0f32; output_row_count * max_col_count]; output_count],
        }
    }

    pub fn input_row_count(&self) -> usize
    { self.input_row_count }

    pub fn output_row_count(&self) -> usize
    { self.output_row_count }

    pub fn max_col_count(&self) -> usize
    { self.max_col_count }
    
    pub fn is_full(&self) -> bool
    { self.elems.len() >= self.max_col_count }
    
    pub fn elems(&self) -> &[T]
    { self.elems.as_slice() }

    pub fn clear(&mut self)
    { self.elems.clear(); }
    
    pub fn push(&mut self, elem: T)
    { self.elems.push(elem); }
    
    pub fn do_elems<F, G>(&mut self, intr_checker: &dyn IntrCheck, mut f: F, mut g: G) -> Result<(), Interruption>
        where F: FnMut(&T, &mut [f32], &mut [Vec<f32>], usize, usize),
            G: FnMut(Matrix, &[Matrix], &mut [T])
    {
        for i in (0..self.elems.len()).step_by(self.max_col_count) {
            intr_checker.check()?;
            let col_count = min(self.max_col_count, self.elems.len() - i);
            for j in 0..col_count {
                f(&self.elems[i + j], self.input_buf.as_mut_slice(), self.output_bufs.as_mut_slice(), j, col_count);
            }
            let input = Matrix::new_with_elems(self.input_row_count, col_count, &self.input_buf[0..(self.input_row_count * col_count)]);
            let outputs: Vec<Matrix> = self.output_bufs.iter().map(|output_buf| {
                    Matrix::new_with_elems(self.output_row_count, col_count, &output_buf[0..(self.output_row_count * col_count)])
            }).collect();
            g(input, outputs.as_slice(), &mut self.elems[i..(i + col_count)]);
        }
        Ok(())
    }
}
