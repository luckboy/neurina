//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;
use crate::shared::network::*;
use crate::shared::xavier_init::*;
use crate::trainer::net_create::*;

#[derive(Copy, Clone, Debug)]
pub struct XavierNetworkFactory
{
    middle_count: usize,
}

impl XavierNetworkFactory
{
    pub fn new(middle_count: usize) -> Self
    { XavierNetworkFactory { middle_count, } }
}

impl NetCreate<Network> for XavierNetworkFactory
{
    fn create(&self, input_count: usize, output_count: usize) -> Network
    {
        let mut iw_elems = vec![0.0f32; self.middle_count * input_count];
        xavier_init(iw_elems.as_mut_slice(), input_count, self.middle_count);
        let iw = Matrix::new_with_elems(self.middle_count, input_count, iw_elems.as_slice());
        let mut ib_elems = vec![0.0f32; self.middle_count];
        xavier_init(ib_elems.as_mut_slice(), input_count, self.middle_count);
        let ib = Matrix::new_with_elems(self.middle_count, 1, ib_elems.as_slice());
        let mut sw_elems = vec![0.0f32; self.middle_count * self.middle_count];
        xavier_init(sw_elems.as_mut_slice(), self.middle_count, self.middle_count);
        let sw = Matrix::new_with_elems(self.middle_count, self.middle_count, sw_elems.as_slice());
        let mut sb_elems = vec![0.0f32; self.middle_count];
        xavier_init(sb_elems.as_mut_slice(), self.middle_count, self.middle_count);
        let sb = Matrix::new_with_elems(self.middle_count, 1, sb_elems.as_slice());
        let mut pw_elems = vec![0.0f32; self.middle_count * self.middle_count];
        xavier_init(pw_elems.as_mut_slice(), self.middle_count, self.middle_count);
        let pw = Matrix::new_with_elems(self.middle_count, self.middle_count, pw_elems.as_slice());
        let mut pb_elems = vec![0.0f32; self.middle_count];
        xavier_init(pb_elems.as_mut_slice(), self.middle_count, self.middle_count);
        let pb = Matrix::new_with_elems(self.middle_count, 1, pb_elems.as_slice());
        let mut ow_elems = vec![0.0f32; output_count * self.middle_count];
        xavier_init(ow_elems.as_mut_slice(), self.middle_count, output_count);
        let ow = Matrix::new_with_elems(output_count, self.middle_count, ow_elems.as_slice());
        let mut ob_elems = vec![0.0f32; output_count];
        xavier_init(ob_elems.as_mut_slice(), self.middle_count, output_count);
        let ob = Matrix::new_with_elems(output_count, 1, ob_elems.as_slice());
        Network::new(iw, ib, sw, sb, pw, pb, ow, ob)
    }
}
