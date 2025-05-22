//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::chess::Board;

pub const MAX_EVAL_VALUE: i32 = 32767;
pub const MIN_EVAL_VALUE: i32 = -32767;

pub const MAX_EVAL_MATE_VALUE: i32 = MAX_EVAL_VALUE - 384;
pub const MIN_EVAL_MATE_VALUE: i32 = MIN_EVAL_VALUE + 384;

pub const MAX_EVAL_MIDDLE_MATE_VALUE: i32 = MAX_EVAL_VALUE - 256;
pub const MIN_EVAL_MIDDLE_MATE_VALUE: i32 = MIN_EVAL_VALUE + 256;

pub const MAX_EVAL_ROOT_MATE_VALUE: i32 = MAX_EVAL_VALUE - 128;
pub const MIN_EVAL_ROOT_MATE_VALUE: i32 = MIN_EVAL_VALUE + 128;

pub trait Eval
{
    fn evaluate(&self, board: &Board) -> i32;
}
