//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;

pub struct Network
{
    iw: Matrix,
    ib: Matrix,
    sw: Matrix,
    sb: Matrix,
    pw: Matrix,
    pb: Matrix,
    ow: Matrix,
    ob: Matrix,
}

impl Network
{
    pub fn new(iw: Matrix, ib: Matrix, sw: Matrix, sb: Matrix, pw: Matrix, pb: Matrix, ow: Matrix, ob: Matrix) -> Self
    { Network { iw, ib, sw, sb, pw, pb, ow, ob, } }
    
    pub fn iw(&self) -> &Matrix
    { &self.iw }

    pub fn ib(&self) -> &Matrix
    { &self.ib }
    
    pub fn sw(&self) -> &Matrix
    { &self.sw }
    
    pub fn sb(&self) -> &Matrix
    { &self.sb }
    
    pub fn pw(&self) -> &Matrix
    { &self.pw }
    
    pub fn pb(&self) -> &Matrix
    { &self.pb }

    pub fn ow(&self) -> &Matrix
    { &self.ow }
    
    pub fn ob(&self) -> &Matrix
    { &self.ob }
    
    pub fn compute<HF, OF>(&self, i: &Matrix, depth: usize, pv_count: usize, mut hf: HF, mut of: OF)
        where HF: FnMut(Matrix), OF: FnMut(Matrix)
    {
        let ib = if i.col_count() > 1 { self.ib.repeat(i.col_count()) } else { self.ib.clone() };
        let sb = if i.col_count() > 1 { self.sb.repeat(i.col_count()) } else { self.sb.clone() };
        let pb = if i.col_count() > 1 { self.pb.repeat(i.col_count()) } else { self.pb.clone() };
        let ob = if i.col_count() > 1 { self.ob.repeat(i.col_count()) } else { self.ob.clone() };
        let mut z = &self.iw * i + &ib;
        let mut h = z.tanh();
        hf(h.clone());
        for _ in 0..depth {
            z = &self.sw * &h + &sb;
            h = z.tanh();
            hf(h.clone());
        }
        for _ in 0..pv_count {
            z = &self.pw * &h + &pb;
            h = z.tanh();
            hf(h.clone());
            let o = &self.ow * &h + &ob;
            of(o);
        }
    }
}
