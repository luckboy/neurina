//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use crate::chess::Move;
use crate::chess::MoveChain;
use crate::shared::intr_check::*;
use crate::shared::Interruption;

pub trait Search
{
    fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>;
    
    fn search(&self, move_chain: &mut MoveChain, depth: usize, search_moves: &Option<Vec<Move>>) -> Result<(i32, u64, u64, Vec<Move>), Interruption>;
    
    fn move_count_to_checkmate(&self, value: i32, depth: usize) -> Option<usize>;
    
    fn min_depth(&self) -> usize;
}
