//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::trainer::data_sample::*;

pub trait Sample
{
    fn samples(&self, sample: &DataSample) -> Option<Vec<DataSample>>;
}
