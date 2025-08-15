//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use rand::random_range;
use crate::chess::Board;
use crate::engine::eval::*;

#[derive(Clone)]
pub struct RandomEvalFun
{
    eval_fun: Arc<dyn Eval + Send + Sync>,
    range: i32,
}

impl RandomEvalFun
{
    pub fn new(eval_fun: Arc<dyn Eval + Send + Sync>, range: i32) -> Self
    { RandomEvalFun { eval_fun, range, } }
}

impl Eval for RandomEvalFun
{
    fn evaluate(&self, board: &Board) -> i32
    { self.eval_fun.evaluate(board) + random_range(-self.range..=self.range) }
}
