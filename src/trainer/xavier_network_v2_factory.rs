//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;
use crate::shared::network_v2::*;
use crate::shared::xavier_init::*;
use crate::trainer::net_create::*;

#[derive(Copy, Clone, Debug)]
pub struct XavierNetworkV2Factory
{
    middle_count: usize,
}

impl XavierNetworkV2Factory
{
    pub fn new(middle_count: usize) -> Self
    { XavierNetworkV2Factory { middle_count, } }
}

impl NetCreate<NetworkV2> for XavierNetworkV2Factory
{
    fn create(&self, input_count: usize, output_count: usize) -> NetworkV2
    {
        let mut iw_elems = vec![0.0f32; self.middle_count * input_count];
        xavier_init(iw_elems.as_mut_slice(), input_count, self.middle_count);
        let iw = Matrix::new_with_elems(self.middle_count, input_count, iw_elems.as_slice());
        let mut ib_elems = vec![0.0f32; self.middle_count];
        xavier_init(ib_elems.as_mut_slice(), input_count, self.middle_count);
        let ib = Matrix::new_with_elems(self.middle_count, 1, ib_elems.as_slice());
        let mut ow_elems = vec![0.0f32; output_count * self.middle_count];
        xavier_sqrt_init(ow_elems.as_mut_slice(), self.middle_count, output_count);
        let ow = Matrix::new_with_elems(output_count, self.middle_count, ow_elems.as_slice());
        let mut ob_elems = vec![0.0f32; output_count];
        xavier_sqrt_init(ob_elems.as_mut_slice(), self.middle_count, output_count);
        let ob = Matrix::new_with_elems(output_count, 1, ob_elems.as_slice());
        NetworkV2::new(iw, ib, ow, ob)
    }
}
