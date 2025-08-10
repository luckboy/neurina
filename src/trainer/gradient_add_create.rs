//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use std::sync::Arc;
use crate::shared::converter::*;
use crate::shared::intr_check::*;

/// A trait of factory of gradient adder.
///
/// This trait provides method that creates a gradient adder.
pub trait GradientAddCreate<T>
{
    /// Creates a gradient adder.
    fn create(&self, intr_checker: Arc<dyn IntrCheck + Send + Sync>, converter: Converter) -> Result<T>;
}
