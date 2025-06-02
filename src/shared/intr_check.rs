//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::time::Duration;
use std::time::Instant;
use crate::shared::Interruption;

pub trait IntrCheck
{
    fn check(&self) -> Result<(), Interruption>;
    
    fn set_timeout(&self, _now: Instant, _duration: Duration) -> bool
    { false }

    fn unset_timeout(&self) -> bool
    { false }

    fn start(&self) -> bool
    { false }

    fn stop(&self) -> bool
    { false }

    fn set_first(&self, _is_first: bool) -> bool
    { false }
}

#[derive(Copy, Clone, Debug)]
pub struct EmptyIntrChecker;

impl EmptyIntrChecker
{
    pub fn new() -> Self
    { EmptyIntrChecker }
}

impl IntrCheck for EmptyIntrChecker
{
    fn check(&self) -> Result<(), Interruption>
    { Ok(()) }
}
