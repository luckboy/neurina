//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::time::Duration;
use std::time::Instant;
use libc::SIGINT;
use libc::c_int;
use libc::sighandler_t;
use libc::signal;
use crate::shared::intr_check::*;
use crate::shared::Interruption;

static mut INTERRUPTION_STOP_FLAG: bool = false;

extern "C" fn neurina_signal_handler(_sig: c_int)
{ unsafe { INTERRUPTION_STOP_FLAG = true; } }

pub fn initialize_intr_checker()
{ unsafe { signal(SIGINT, neurina_signal_handler as sighandler_t); } }

pub fn start_intr_checker()
{ unsafe { INTERRUPTION_STOP_FLAG = false; } }

pub fn stop_intr_checker()
{ unsafe { INTERRUPTION_STOP_FLAG = true; } }

#[derive(Copy, Clone, Debug)]
pub struct IntrChecker;

impl IntrChecker
{
    pub fn new() -> Self
    { IntrChecker }
}

impl IntrCheck for IntrChecker
{
    fn check(&self) -> Result<(), Interruption>
    {
        if unsafe { INTERRUPTION_STOP_FLAG } {
            Err(Interruption::CtrlC)
        } else {
            Ok(())
        }
    }

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
