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

/// A search trait.
///
/// This trait provides method that searches a game tree. 
pub trait Search
{
    /// Returns the interruption checker.
    fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>;
    
    /// Searches a game tree.
    ///
    /// The search moves are moves from which the search begins. This method returns a value, a
    /// number of nodes of middle search, a number of nodes, and a principal variation.
    fn search(&self, move_chain: &mut MoveChain, depth: usize, search_moves: &Option<Vec<Move>>) -> Result<(i32, u64, u64, Vec<Move>), Interruption>;
    
    /// Calculates a number of moves to checkmate.
    ///
    /// The number of moves to checkmate is calculated from a search value and a depth.
    fn move_count_to_checkmate(&self, value: i32, depth: usize) -> Option<usize>;
    
    /// Returns a minimal depth that can be used in search.
    fn min_depth(&self) -> usize;
}
