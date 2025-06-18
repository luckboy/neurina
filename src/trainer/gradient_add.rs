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

pub trait GradientAdd
{
    fn intr_checker(&self) -> &Arc<dyn IntrCheck + Send + Sync>;
    
    fn samples_are_full(&self, sample_count: usize) -> bool;

    fn start(&self);
    
    fn compute(&self, samples: &mut [DataSample], move_count: usize, are_gradients: bool) -> TrainerResult<(u64, u64)>;
    
    fn divide(&self) -> TrainerResult<()>;
}
