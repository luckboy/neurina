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

pub const EVAL_MATE_VALUE: i32 = MIN_EVAL_VALUE + 256;

pub trait Eval
{
    fn evaluate(&self, board: &Board) -> i32;
}
