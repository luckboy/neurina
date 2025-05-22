//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;

pub trait Net
{
    fn compute<HF, OF>(&self, i: &Matrix, depth: usize, pv_count: usize, hf: HF, of: OF)
        where HF: FnMut(Matrix), OF: FnMut(Matrix);
    
    fn backpropagate(&self, i: &Matrix, hs: &[Matrix], os: &[Matrix], ys: &[Matrix], one: &Matrix) -> Self;
}
