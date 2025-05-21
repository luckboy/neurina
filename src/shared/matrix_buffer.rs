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

struct MatrixBufferInner<T>
{
    input_row_count: usize,
    output_row_count: usize,
    max_col_count: usize,
    input_buf: Vec<f32>,
    output_bufs: Vec<Vec<f32>>,
    middle_buf: T,
}

impl<T> MatrixBufferInner<T>
{
   fn new(input_row_count: usize, output_row_count: usize, max_col_count: usize, output_count: usize, middle_buf: T) -> Self
   {
        MatrixBufferInner {
            input_row_count,
            output_row_count,
            max_col_count,
            input_buf: vec![0.0f32; input_row_count * max_col_count],
            output_bufs: vec![vec![0.0f32; output_row_count * max_col_count]; output_count],
            middle_buf,
        }
    }

    fn do_elems<U, F, G>(&mut self, elems: &mut [U], intr_checker: &dyn IntrCheck, mut f: F, mut g: G) -> Result<(), Interruption>
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
            let outputs: Vec<Matrix> = self.output_bufs.iter().map(|output_buf| {
                    Matrix::new_with_elems(self.output_row_count, col_count, &output_buf[0..(self.output_row_count * col_count)])
            }).collect();
            g(input, outputs.as_slice(), &mut self.middle_buf, &mut elems[i..(i + col_count)]);
        }
        Ok(())
    }
}

pub struct MatrixBuffer<T, U>
{
    elems: Vec<T>,
    inner: MatrixBufferInner<U>,
}

impl<T, U> MatrixBuffer<T, U>
{
    pub fn new(input_row_count: usize, output_row_count: usize, max_col_count: usize, output_count: usize, middle_buf: U) -> Self
    {
        MatrixBuffer {
            elems: Vec::new(),
            inner: MatrixBufferInner::new(input_row_count, output_row_count, max_col_count, output_count, middle_buf),
        }
    }

    pub fn input_row_count(&self) -> usize
    { self.inner.input_row_count }

    pub fn output_row_count(&self) -> usize
    { self.inner.output_row_count }

    pub fn max_col_count(&self) -> usize
    { self.inner.max_col_count }
    
    pub fn is_full(&self) -> bool
    { self.elems.len() >= self.inner.max_col_count }
    
    pub fn elems(&self) -> &[T]
    { self.elems.as_slice() }

    pub fn clear(&mut self)
    { self.elems.clear(); }
    
    pub fn push(&mut self, elem: T)
    { self.elems.push(elem); }
    
    pub fn do_elems_for_slice<F, G>(&mut self, elems: &mut [T], intr_checker: &dyn IntrCheck, f: F, g: G) -> Result<(), Interruption>
        where F: FnMut(&T, &mut [f32], &mut [Vec<f32>], usize, usize),
            G: FnMut(Matrix, &[Matrix], &mut U, &mut [T])
    { self.inner.do_elems(elems, intr_checker, f, g) }

    pub fn do_elems<F, G>(&mut self, intr_checker: &dyn IntrCheck, f: F, g: G) -> Result<(), Interruption>
        where F: FnMut(&T, &mut [f32], &mut [Vec<f32>], usize, usize),
            G: FnMut(Matrix, &[Matrix], &mut U, &mut [T])
    { self.inner.do_elems(self.elems.as_mut_slice(), intr_checker, f, g) }
}
