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

pub(crate) mod lichess_puzzles;
pub(crate) mod print;
pub(crate) mod printer;
pub(crate) mod selector;

pub use lichess_puzzles::*;
pub use print::*;
pub use printer::*;
pub use selector::*;

/// An enumeration of selector error.
#[derive(Debug)]
pub enum SelectorError
{
    /// An interruption error.
    Interruption(Interruption),
    /// An input/output error.
    Io(std::io::Error),
}

impl error::Error for SelectorError
{}

impl fmt::Display for SelectorError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            SelectorError::Interruption(Interruption::Timeout) => write!(f, "interrupted by timeout"),
            SelectorError::Interruption(Interruption::Stop) => write!(f, "interrupted by stop"),
            SelectorError::Interruption(Interruption::CtrlC) => write!(f, "interrupted by ctrl-c"),
            SelectorError::Io(err) => write!(f, "{}", err),
        }
    }
}

/// A type of selector result.
pub type SelectorResult<T> = Result<T, SelectorError>;
