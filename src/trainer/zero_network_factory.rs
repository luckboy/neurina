//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;
use crate::shared::network::*;
use crate::trainer::net_create::*;

#[derive(Copy, Clone, Debug)]
pub struct ZeroNetworkFactory
{
    middle_count: usize,
}

impl ZeroNetworkFactory
{
    pub fn new(middle_count: usize) -> Self
    { ZeroNetworkFactory { middle_count, } }
}

impl NetCreate<Network> for ZeroNetworkFactory
{
    fn create(&self, input_count: usize, output_count: usize) -> Network
    {
        let iw = Matrix::new(self.middle_count, input_count);
        let ib = Matrix::new(self.middle_count, 1);
        let sw = Matrix::new(self.middle_count, self.middle_count);
        let sb = Matrix::new(self.middle_count, 1);
        let pw = Matrix::new(self.middle_count, self.middle_count);
        let pb = Matrix::new(self.middle_count, 1);
        let ow = Matrix::new(output_count, self.middle_count);
        let ob = Matrix::new(output_count, 1);
        Network::new(iw, ib, sw, sb, pw, pb, ow, ob)
    }
}
