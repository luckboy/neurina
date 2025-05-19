//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;

//
// output layer
//   ^ ^ ^ ^
//   |X|X|X|  ow * h[depth + pv_count] + ob
//   pv layer
//   ^ ^ ^ ^
//   |X|X|X|  pw * h[depth + pv_count - 1] + pb
//   pv layer -> ow * h[depth + pv_count - 1] + ob -> output layer
//     ...
//   pv layer -> ow * h[depth + 1] + ob -> output layer
//   ^ ^ ^ ^
//   |X|X|X|  pw * h[depth] + pb
//   pv layer
//   ^ ^ ^ ^
//   |X|X|X|  sw * h[depth - 1] + sb
// search layer
//     ...
// search layer
//   ^ ^ ^ ^
//   |X|X|X|  sw * h[0] + sb
// search layer
//   ^ ^ ^ ^
//   |X|X|X|  iw * i + ib
// input layer
//

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
    
    pub fn backpropagate(&self, i: &Matrix, hs: &[Matrix], os: &[Matrix], ys: &[Matrix], one: &Matrix) -> Network
    {
        let mut dj_dnet: Option<Network> = None;
        for (pv_count_1, (o, y)) in os.iter().zip(ys).enumerate() {
            let pv_count = pv_count_1 + 1;
            let depth = hs.len() - ys.len() - 1;
            let mut j = hs.len() - (os.len() - pv_count) - 1;
            let dj_do = o.softmax() - y;
            let dj_dow = &dj_do * hs[j].t();
            let dj_dob = &dj_do * one;
            // dj/dz = (pw^T * dj/do) (*) phi'(z)
            let mut dj_dh = self.ow.t() * &dj_do;
            let mut dj_dz = dj_dh.mul_elems(&(hs[j].mul_elems(&hs[j]).rsub(1.0)));
            j -= 1;
            let (dj_dpw, dj_dpb) = if pv_count > 1 {
                // dj/dpw = dj/dz2 * phi(z)^T
                // dj/dpb = dj/dz2
                let mut tmp_dj_dpw = &dj_dz * &hs[j].t();
                let mut tmp_dj_dpb = &dj_dz * one;
                let mut tmp = dj_dz.clone();
                for _ in 1..pv_count {
                    // dj/dpw += ((pw^T * dj/dz2) (*) phi'(z)) * h^T
                    // dj/dpb += (pw^T * dj/dz2) (*) phi'(z)
                    tmp = self.pw.t() * &tmp;
                    tmp = tmp.mul_elems(&(hs[j].mul_elems(&hs[j]).rsub(1.0)));
                    j -= 1;
                    tmp_dj_dpw += &tmp * hs[j].t();
                    tmp_dj_dpb += &tmp * one;
                }
                dj_dz = tmp;
                (tmp_dj_dpw, tmp_dj_dpb)
            } else {
                let tmp_dj_dpw = &dj_dz * hs[j].t();
                let tmp_dj_dpb = &dj_dz * one;
                (tmp_dj_dpw, tmp_dj_dpb)
            };
            // dj/dz = (pw^T * dj/dz2) (*) phi'(z)
            dj_dh = self.pw.t() * &dj_dz;
            dj_dz = dj_dh.mul_elems(&(hs[j].mul_elems(&hs[j]).rsub(1.0)));
            j -= 1;
            let (dj_dsw, dj_dsb) = if depth > 1 {
                // dj/dsw = dj/dz2 * phi(z)^T
                // dj/dsb = dj/dz2
                let mut tmp_dj_dsw = &dj_dz * &hs[j].t();
                let mut tmp_dj_dsb = &dj_dz * one;
                let mut tmp = dj_dz.clone();
                for _ in 1..depth {
                    // dj/dsw += ((pw^T * dj/dz2) (*) phi'(z)) * h^T
                    // dj/dsb += (pw^T * dj/dz2) (*) phi'(z)
                    tmp = self.pw.t() * &tmp;
                    tmp = tmp.mul_elems(&(hs[j].mul_elems(&hs[j]).rsub(1.0)));
                    j -= 1;
                    tmp_dj_dsw += &tmp * hs[j].t();
                    tmp_dj_dsb += &tmp * one;
                }
                dj_dz = tmp;
                (tmp_dj_dsw, tmp_dj_dsb)
            } else {
                let tmp_dj_dsw = &dj_dz * hs[j].t();
                let tmp_dj_dsb = &dj_dz * one;
                (tmp_dj_dsw, tmp_dj_dsb)
            };
            // dj/dz = (pw^T * dj/dz2) (*) phi'(z)
            dj_dh = self.pw.t() * &dj_dz;
            dj_dz = dj_dh.mul_elems(&(hs[j].mul_elems(&hs[j]).rsub(1.0)));
            let dj_diw = &dj_dz * i.t();
            let dj_dib = &dj_dz * one;
            match &mut dj_dnet {
                Some(dj_dnet) => {
                    dj_dnet.iw += dj_diw;
                    dj_dnet.ib += dj_dib;
                    dj_dnet.sw += dj_dsw;
                    dj_dnet.sb += dj_dsb;
                    dj_dnet.pw += dj_dpw;
                    dj_dnet.pb += dj_dpb;
                    dj_dnet.ow += dj_dow;
                    dj_dnet.ob += dj_dob;
                },
                None => {
                    dj_dnet = Some(Network {
                            iw: dj_diw,
                            ib: dj_dib,
                            sw: dj_dsw,
                            sb: dj_dsb,
                            pw: dj_dpw,
                            pb: dj_dpb,
                            ow: dj_dow,
                            ob: dj_dob,
                    });
                },
            }
        }
        dj_dnet.unwrap()
    }
}
