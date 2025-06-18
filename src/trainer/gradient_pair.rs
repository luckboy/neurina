//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::trainer::TrainerResult;

pub trait GradientPair<T>
{
    fn network_in<U, F>(&self, f: F) -> U
        where F: FnOnce(&mut T) -> U;
    
    fn gradient_in<U, F>(&self, f: F) -> TrainerResult<U>
        where F: FnOnce(&T) -> U;

    fn network_and_gradient_in<U, F>(&self, f: F) -> TrainerResult<U>
        where F: FnOnce(&mut T, &T) -> U;
}
