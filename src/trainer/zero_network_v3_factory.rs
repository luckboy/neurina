//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;
use crate::shared::network_v3::*;
use crate::trainer::net_create::*;

#[derive(Copy, Clone, Debug)]
pub struct ZeroNetworkV3Factory
{
    middle_count: usize,
}

impl ZeroNetworkV3Factory
{
    pub fn new(middle_count: usize) -> Self
    { ZeroNetworkV3Factory { middle_count, } }
}

impl NetCreate<NetworkV3> for ZeroNetworkV3Factory
{
    fn create(&self, input_count: usize, output_count: usize) -> NetworkV3
    {
        let iw = Matrix::new(self.middle_count, input_count);
        let ib = Matrix::new(self.middle_count, 1);
        let sw = Matrix::new(self.middle_count, self.middle_count);
        let sb = Matrix::new(self.middle_count, 1);
        let pw = Matrix::new(self.middle_count, self.middle_count);
        let pb = Matrix::new(self.middle_count, 1);
        let ow = Matrix::new(output_count, self.middle_count);
        let ob = Matrix::new(output_count, 1);
        NetworkV3::new(iw, ib, sw, sb, pw, pb, ow, ob)
    }
}
