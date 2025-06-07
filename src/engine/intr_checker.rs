//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use crate::shared::intr_check::*;
use crate::shared::Interruption;

#[derive(Debug)]
pub struct IntrChecker
{
    timeout_pair: Mutex<Option<(Instant, Duration)>>,
    is_stopped: AtomicBool,
    has_first: AtomicBool,
}

impl IntrChecker
{
    pub fn new() -> Self
    {
        IntrChecker {
            timeout_pair: Mutex::new(None),
            is_stopped: AtomicBool::new(true),
            has_first: AtomicBool::new(true),
        }
    }
}

impl IntrCheck for IntrChecker
{
    fn check(&self) -> Result<(), Interruption>
    {
        if !self.has_first.load(Ordering::SeqCst) {
            if self.is_stopped.load(Ordering::SeqCst) {
                return Err(Interruption::Stop);
            }
            let timeout_pair_g = self.timeout_pair.lock().unwrap();
            match *timeout_pair_g {
                Some((now, duration)) => {
                    if now.elapsed() >= duration {
                        return Err(Interruption::Timeout);
                    }
                },
                None => (),
            }
        }
        Ok(())
    }

    fn set_timeout(&self, now: Instant, duration: Duration) -> bool
    {
        let mut timeout_pair_g = self.timeout_pair.lock().unwrap();
        *timeout_pair_g = Some((now, duration));
        true
    }

    fn unset_timeout(&self) -> bool
    {
        let mut timeout_pair_g = self.timeout_pair.lock().unwrap();
        *timeout_pair_g = None;
        true
    }

    fn start(&self) -> bool
    {
        self.is_stopped.store(false, Ordering::SeqCst);
        true
    }

    fn stop(&self) -> bool
    {
        self.is_stopped.store(true, Ordering::SeqCst);
        true
    }

    fn set_first(&self, is_first: bool) -> bool
    {
        self.has_first.store(is_first, Ordering::SeqCst);
        true
    }
}
