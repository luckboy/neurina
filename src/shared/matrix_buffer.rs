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
    input_row_count: usize,
    output_row_count: usize,
    max_col_count: usize,
    input_buf: Vec<f32>,
    output_bufs: Vec<Vec<f32>>,
    middle_buf: T,
}

impl<T> MatrixBuffer<T>
{
    pub fn new(input_row_count: usize, output_row_count: usize, max_col_count: usize, max_output_count: usize, middle_buf: T) -> Self
    {
        MatrixBuffer {
            input_row_count,
            output_row_count,
            max_col_count,
            input_buf: vec![0.0f32; input_row_count * max_col_count],
            output_bufs: vec![vec![0.0f32; output_row_count * max_col_count]; max_output_count],
            middle_buf,
        }
    }

    pub fn input_row_count(&self) -> usize
    { self.input_row_count }

    pub fn output_row_count(&self) -> usize
    { self.output_row_count }

    pub fn max_col_count(&self) -> usize
    { self.max_col_count }

    pub fn max_output_count(&self) -> usize
    { self.output_bufs.len() }
    
    pub fn resize_output_bufs(&mut self, max_output_count: usize)
    { self.output_bufs.resize(max_output_count, vec![0.0f32; self.output_row_count * self.max_col_count]); }
    
    pub fn elems_are_full(&self, elem_count: usize) -> bool
    { elem_count >= self.max_col_count }
    
    pub fn do_elems<U, F, G>(&mut self, elems: &mut [U], output_count: usize, intr_checker: &dyn IntrCheck, mut f: F, mut g: G) -> Result<(), Interruption>
        where F: FnMut(&U, &mut [f32], &mut [Vec<f32>], usize, usize),
            G: FnMut(Matrix, &[Matrix], &mut T, &mut [U])
    {
        for i in (0..elems.len()).step_by(self.max_col_count) {
            intr_checker.check()?;
            let col_count = min(self.max_col_count, elems.len() - i);
            for j in 0..col_count {
                f(&elems[i + j], self.input_buf.as_mut_slice(), self.output_bufs.as_mut_slice(), j, col_count);
            }
            let input = Matrix::new_with_elems(self.input_row_count, col_count, &self.input_buf[0..(self.input_row_count * col_count)]);
            let outputs: Vec<Matrix> = (0..output_count).map(|j| {
                    Matrix::new_with_elems(self.output_row_count, col_count, &self.output_bufs[j][0..(self.output_row_count * col_count)])
            }).collect();
            g(input, outputs.as_slice(), &mut self.middle_buf, &mut elems[i..(i + col_count)]);
        }
        Ok(())
    }
}
