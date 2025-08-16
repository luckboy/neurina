//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::error;
use std::fmt;
use crate::shared::Interruption;

pub mod algorithms;

pub(crate) mod algorithm;
pub(crate) mod data_sample;
pub(crate) mod gradient_add;
pub(crate) mod gradient_add_create;
pub(crate) mod gradient_adder;
pub(crate) mod gradient_pair;
pub(crate) mod io;
pub(crate) mod lichess_puzzles;
pub(crate) mod multi_sampler;
pub(crate) mod net_create;
pub(crate) mod one_gradient_adder;
pub(crate) mod print;
pub(crate) mod printer;
pub(crate) mod sample;
pub(crate) mod single_sampler;
pub(crate) mod trainer;
pub(crate) mod xavier_network_factory;
pub(crate) mod zero_network_factory;

pub use algorithm::*;
pub use data_sample::*;
pub use gradient_add::*;
pub use gradient_add_create::*;
pub use gradient_adder::*;
pub use gradient_pair::*;
pub use io::*;
pub use lichess_puzzles::*;
pub use multi_sampler::*;
pub use net_create::*;
pub use one_gradient_adder::*;
pub use print::*;
pub use printer::*;
pub use sample::*;
pub use single_sampler::*;
pub use trainer::*;
pub use xavier_network_factory::*;
pub use zero_network_factory::*;

/// An enumeration of trainer error.
#[derive(Debug)]
pub enum TrainerError
{
    /// An interruption error.
    Interruption(Interruption),
    /// No gradient.
    NoGradient,
    /// An input/output error.
    Io(std::io::Error),
}

impl error::Error for TrainerError
{}

impl fmt::Display for TrainerError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            TrainerError::Interruption(Interruption::Timeout) => write!(f, "interrupted by timeout"),
            TrainerError::Interruption(Interruption::Stop) => write!(f, "interrupted by stop"),
            TrainerError::Interruption(Interruption::CtrlC) => write!(f, "interrupted by ctrl-c"),
            TrainerError::NoGradient => write!(f, "no gradient"),
            TrainerError::Io(err) => write!(f, "{}", err),
        }
    }
}

/// A type of trainer result.
pub type TrainerResult<T> = Result<T, TrainerError>;
