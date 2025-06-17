//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::trainer::gradient_add::*;
use crate::trainer::TrainerResult;

pub trait Algorithm
{
    fn gradient_adder(&self) -> &(dyn GradientAdd + Send + Sync);
    
    fn save(&self) -> TrainerResult<()>;

    fn do_alg(&self) -> TrainerResult<()>;
}
