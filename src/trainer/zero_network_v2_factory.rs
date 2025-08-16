//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;
use crate::shared::network_v2::*;
use crate::trainer::net_create::*;

#[derive(Copy, Clone, Debug)]
pub struct ZeroNetworkV2Factory
{
    middle_count: usize,
}

impl ZeroNetworkV2Factory
{
    pub fn new(middle_count: usize) -> Self
    { ZeroNetworkV2Factory { middle_count, } }
}

impl NetCreate<NetworkV2> for ZeroNetworkV2Factory
{
    fn create(&self, input_count: usize, output_count: usize) -> NetworkV2
    {
        let iw = Matrix::new(self.middle_count, input_count);
        let ib = Matrix::new(self.middle_count, 1);
        let ow = Matrix::new(output_count, self.middle_count);
        let ob = Matrix::new(output_count, 1);
        NetworkV2::new(iw, ib, ow, ob)
    }
}
