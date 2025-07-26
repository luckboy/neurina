//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::Board;

/// A maximal evaluation value.
pub const MAX_EVAL_VALUE: i32 = 32767;
/// A minimal evaluation value.
pub const MIN_EVAL_VALUE: i32 = -32767;

/// A maximal evaluation value of checkmate.
pub const MAX_EVAL_MATE_VALUE: i32 = MAX_EVAL_VALUE - 384;
/// A minimal evaluation value of checkmate.
pub const MIN_EVAL_MATE_VALUE: i32 = MIN_EVAL_VALUE + 384;

/// A maximal evaluation value of checkmate of middle search.
pub const MAX_EVAL_MIDDLE_MATE_VALUE: i32 = MAX_EVAL_VALUE - 256;
/// A minimal evaluation value of checkmate of middle search.
pub const MIN_EVAL_MIDDLE_MATE_VALUE: i32 = MIN_EVAL_VALUE + 256;

/// A maximal evaluation value of root checkmate.
pub const MAX_EVAL_ROOT_MATE_VALUE: i32 = MAX_EVAL_VALUE - 128;
/// A minimal evaluation value of root checkmate.
pub const MIN_EVAL_ROOT_MATE_VALUE: i32 = MIN_EVAL_VALUE + 128;

/// A trait of evaluation function.
///
/// The evaulation function evaluates a board.
pub trait Eval
{
    /// Evaluates the board.
    fn evaluate(&self, board: &Board) -> i32;
}
