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

pub(crate) mod algorithm;
pub(crate) mod data_sample;
pub(crate) mod gradient_add;
pub(crate) mod gradient_adder;
pub(crate) mod gradient_pair;
pub(crate) mod multi_sampler;
pub(crate) mod print;
pub(crate) mod sample;
pub(crate) mod single_sampler;
pub(crate) mod trainer;

pub use algorithm::*;
pub use data_sample::*;
pub use gradient_add::*;
pub use gradient_adder::*;
pub use gradient_pair::*;
pub use multi_sampler::*;
pub use print::*;
pub use sample::*;
pub use single_sampler::*;
pub use trainer::*;

#[derive(Debug)]
pub enum TrainerError
{
    Interruption(Interruption),
    NoGradient,
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

pub type TrainerResult<T> = Result<T, TrainerError>;
