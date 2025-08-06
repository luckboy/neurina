//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use crate::trainer::gradient_add::*;
use crate::trainer::TrainerResult;

pub const NETWORK_NAME: &'static str = "neurina.nnet";
pub const NETWORK_NAME_PREFIX: &'static str = "neurina";
pub const NETWORK_NAME_SUFFIX: &'static str = ".nnet";

pub const PARAMS_NAME: &'static str = "params.toml";

pub const STATE_NAME: &'static str = "state.toml";
pub const STATE_NAME_PREFIX: &'static str = "state";
pub const STATE_NAME_SUFFIX: &'static str = ".toml";

/// An algorithm trait.
///
/// The algorithm determines how a neural network will be trained.
pub trait Algorithm
{
    /// Returns the gradient adder.
    fn gradient_adder(&self) -> &(dyn GradientAdd + Send + Sync);

    /// Returns the epoch number.
    fn epoch(&self) -> usize;
    
    /// Saves an epoch state and a computed neural network.
    fn save(&self) -> Result<()>;

    /// Performs the algorithm.
    fn do_algorithm(&self) -> TrainerResult<()>;
}
