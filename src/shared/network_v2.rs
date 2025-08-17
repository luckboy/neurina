//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io;
use std::path::Path;
use crate::matrix::Matrix;
use crate::shared::io::*;
use crate::shared::net::*;
use crate::shared::Interruption;

#[derive(Clone, Debug)]
pub struct NetworkV2
{
    iw: Matrix,
    ib: Matrix,
    ow: Matrix,
    ob: Matrix,
}

impl NetworkV2
{
    pub fn new(iw: Matrix, ib: Matrix, ow: Matrix, ob: Matrix) -> Self
    { NetworkV2 { iw, ib, ow, ob, } }
    
    pub fn iw(&self) -> &Matrix
    { &self.iw }

    pub fn ib(&self) -> &Matrix
    { &self.ib }

    pub fn ow(&self) -> &Matrix
    { &self.ow }

    pub fn ob(&self) -> &Matrix
    { &self.ob }
}

impl Net for NetworkV2
{
    fn compute<HF, OF>(&self, i: &Matrix, depth: usize, pv_count: usize, mut hf: HF, mut of: OF) -> Result<(), Interruption>
        where HF: FnMut(Matrix) -> Result<(), Interruption>,
            OF: FnMut(Matrix)  -> Result<(), Interruption>
    {
        assert_eq!(1, depth);
        assert_eq!(1, pv_count);
        let ib = if i.col_count() > 1 { self.ib.repeat(i.col_count()) } else { self.ib.clone() };
        let ob = if i.col_count() > 1 { self.ob.repeat(i.col_count()) } else { self.ob.clone() };
        let z = &self.iw * i + &ib;
        let h = z.tanh();
        hf(h.clone())?;
        let o = -(self.ow.mul_elems(&self.ow) * &h + ob.mul_elems(&ob));
        of(o)?;
        Ok(())
    }

    fn backpropagate(&self, i: &Matrix, hs: &[Matrix], os: &[Matrix], ys: &[Matrix], one: &Matrix) -> Self
    {
        assert_eq!(1, hs.len());
        assert_eq!(1, os.len());
        assert_eq!(1, ys.len());
        let dj_do = os[0].softmax() - &ys[0];
        let dj_dow = (&dj_do * hs[0].t()).mul_elems(&(&self.ow * -2.0));
        let dj_dob = (&dj_do * one).mul_elems(&(&self.ob * -2.0));
        let dj_dh = (-self.ow.mul_elems(&self.ow)).t() * &dj_do;
        let dj_dz = dj_dh.mul_elems(&(hs[0].mul_elems(&hs[0]).rsub(1.0)));
        let dj_diw = &dj_dz * i.t();
        let dj_dib = &dj_dz * one;
        NetworkV2 {
            iw: dj_diw,
            ib: dj_dib,
            ow: dj_dow,
            ob: dj_dob,
        }
    }

    fn op<F>(&self, network: &Self, mut f: F) -> Self
        where F: FnMut(&Matrix, &Matrix) -> Matrix
    {
        NetworkV2 {
            iw: f(&self.iw, &network.iw),
            ib: f(&self.ib, &network.ib),
            ow: f(&self.ow, &network.ow),
            ob: f(&self.ob, &network.ob),
        }
    }

    fn op_assign<F>(&mut self, network: &Self, mut f: F)
        where F: FnMut(&mut Matrix, &Matrix)
    {
        f(&mut self.iw, &network.iw);
        f(&mut self.ib, &network.ib);
        f(&mut self.ow, &network.ow);
        f(&mut self.ob, &network.ob);
    }

    fn fun<F>(&self, mut f: F) -> Self
        where F: FnMut(&Matrix) -> Matrix
    {
        NetworkV2 {
            iw: f(&self.iw),
            ib: f(&self.ib),
            ow: f(&self.ow),
            ob: f(&self.ob),
        }
    }

    fn check(&self, input_count: usize, output_count: usize) -> bool
    {
        let middle_count: usize = self.iw.row_count();
        if middle_count == 0 { return false; }
        if self.iw.col_count() != input_count { return false; }
        if self.ib.row_count() != middle_count { return false; }
        if self.ib.col_count() != 1 { return false; }
        if self.ow.row_count() != output_count { return false; }
        if self.ow.col_count() != middle_count { return false; }
        if self.ob.row_count() != output_count { return false; }
        if self.ob.col_count() != 1 { return false; }
        true
    }
}

impl Save for NetworkV2
{
    fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()>
    { save_network_v2(path, self) }
}
