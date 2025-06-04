//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::error;
use std::fmt;
use crate::matrix;

pub(crate) mod engine;
pub(crate) mod eval;
pub(crate) mod io;
pub(crate) mod middle_searcher;
pub(crate) mod neural_search;
pub(crate) mod neural_searcher;
pub(crate) mod one_searcher;
pub(crate) mod print;
pub(crate) mod search;
pub(crate) mod simple_eval_fun;
pub(crate) mod thinker;
pub(crate) mod utils;
pub(crate) mod xboard;

pub use engine::*;
pub use eval::*;
pub use io::*;
pub use middle_searcher::*;
pub use neural_search::*;
pub use neural_searcher::*;
pub use one_searcher::*;
pub use print::*;
pub use search::*;
pub use simple_eval_fun::*;
pub use thinker::*;
pub use utils::*;
pub use xboard::*;

#[derive(Debug)]
pub enum LoopError
{
    InvalidNetwork,
    Io(std::io::Error),
    Matrix(matrix::Error),
    UninitializedLoopContext,
}

impl error::Error for LoopError
{}

impl fmt::Display for LoopError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            LoopError::InvalidNetwork => write!(f, "invalid network"),
            LoopError::Io(err) => write!(f, "{}", err),
            LoopError::Matrix(err) => write!(f, "{}", err),
            LoopError::UninitializedLoopContext => write!(f, "uninitialized loop context"),
        }
    }
}

pub type LoopResult<T> = Result<T, LoopError>;
