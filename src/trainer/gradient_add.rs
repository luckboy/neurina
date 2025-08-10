//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
use crate::shared::intr_check::*;
use crate::trainer::data_sample::*;
use crate::trainer::TrainerResult;

/// A trait of gradient adder.
///
/// This trait provides method that adds minibatch gradients to a gradient and other methods. 
pub trait GradientAdd
{
    /// Returns the interruption checker.
    fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>;
    
    /// Returns `true` if the number of samples is greater than or equal to the maximal number of
    /// columns, otherwise `false`.
    fn samples_are_full(&self, sample_count: usize) -> bool;

    /// Prepares to gradient addition.
    fn start(&self);

    /// Computes and adds the minibatch gradients from the data samples to the gradient if the gradient 
    /// flag is enabled.
    ///
    /// This method also computes a result of neural network and returns the number of passed
    /// outputs and the number of all outputs for the neural network.
    fn compute(&self, samples: &mut [DataSample], move_count: usize, are_gradients: bool) -> TrainerResult<(u64, u64)>;

    /// Divides the gradient.
    fn divide(&self) -> TrainerResult<()>;
}
