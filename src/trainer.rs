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
pub(crate) mod print;
pub(crate) mod sample;
pub(crate) mod trainer;

pub use algorithm::*;
pub use data_sample::*;
pub use gradient_add::*;
pub use print::*;
pub use sample::*;
pub use trainer::*;

#[derive(Debug)]
pub enum TrainerError
{
    Interruption(Interruption),
    NoGrandient,
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
            TrainerError::NoGrandient => write!(f, "no grandient"),
            TrainerError::Io(err) => write!(f, "{}", err),
        }
    }
}

pub type TrainerResult<T> = Result<T, TrainerError>;
