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

/// A trait of neural searcher.
///
/// The neural search is a search that uses a neural network. The neural network returns outputs
/// from which the best moves is selected as principal varations.
pub trait NeuralSearch
{
    /// Returns the interruption checker.
    fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>;
    
    /// Searches a game tree from the board by the neural search.
    ///
    /// The principal variations are from a middle search and updated by the neural search.
    fn search(&self, board: &Board, pvs: &mut [Vec<Move>], depth: usize) -> Result<(), Interruption>;
}
