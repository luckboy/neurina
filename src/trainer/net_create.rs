//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// A trait of factory of neural network.
///
/// This trait provides method that creates a neural network.
pub trait NetCreate<T>
{
    /// Creates a neural network.
    fn create(&self, input_count: usize, output_count: usize) -> T;
}
