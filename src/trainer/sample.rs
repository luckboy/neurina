//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::trainer::data_sample::*;

/// A sample trait.
///
/// This trait provides method that creates data samples from one data sample.
pub trait Sample
{
    /// Creates data samples from one data sample.
    fn samples(&self, sample: &DataSample) -> Option<Vec<DataSample>>;
}
