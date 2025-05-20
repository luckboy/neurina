//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cmp::min;
use crate::matrix::Matrix;

pub struct MatrixBuffer<T>
{
    elems: Vec<T>,
    row_count: usize,
    max_col_count: usize,
    buf: Vec<f32>,
}

impl<T> MatrixBuffer<T>
{
    pub fn new(row_count: usize, max_col_count: usize) -> Self
    {
        MatrixBuffer {
            elems: Vec::new(),
            row_count,
            max_col_count,
            buf: vec![0.0f32; row_count * max_col_count],
        }
    }

    pub fn row_count(&self) -> usize
    { self.row_count }

    pub fn max_col_count(&self) -> usize
    { self.max_col_count }
    
    pub fn is_full(&self) -> bool
    { self.elems.len() >= self.max_col_count }
    
    pub fn elems(&self) -> &[T]
    { &self.elems }

    pub fn clear(&mut self)
    { self.elems.clear(); }
    
    pub fn push(&mut self, elem: T)
    { self.elems.push(elem); }
    
    pub fn do_elems<F, G>(&mut self, mut f: F, mut g: G)
        where F: FnMut(&T, &mut [f32], usize, usize), G: FnMut(Matrix, &mut [T])
    {
        for i in (0..self.elems.len()).step_by(self.max_col_count) {
            let col_count = min(self.max_col_count, self.elems.len() - i);
            for j in 0..col_count {
                f(&self.elems[i + j], &mut self.buf, j, col_count);
            }
            g(Matrix::new_with_elems(self.row_count, col_count, &self.buf[0..(self.row_count * col_count)]), &mut self.elems[i..(i + col_count)]);
        }
    }
}
