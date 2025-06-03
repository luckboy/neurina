//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use crate::chess::Board;
use crate::chess::Move;
use crate::shared::intr_check::*;
use crate::shared::Interruption;

pub trait NeuralSearch
{
    fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>;
    
    fn search(&self, board: &Board, pvs: &mut [Vec<Move>], depth: usize) -> Result<(), Interruption>;
}
