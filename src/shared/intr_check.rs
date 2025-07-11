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

/// A trait of interruption checker.
///
/// The interruption checker checks whether an interruption is occurred. Occurence of the
/// interruption informs about for example the searching stopping. The interruption can be occurred
/// for:
///
/// - timeout
/// - enabled stop flag
/// - pressed keys `Ctrl-C`
pub trait IntrCheck
{
    /// Checks whether an interruption is occurred.
    fn check(&self) -> Result<(), Interruption>;
    
    /// Sets the timeout for the interruption checker.
    ///
    /// This method returns `true` if this operation is successful, otherwise `false`.
    fn set_timeout(&self, now: Instant, duration: Duration) -> bool;

    /// Unsets the timeout for the interruption checker.
    ///
    /// This method returns `true` if this operation is successful, otherwise `false`.
    fn unset_timeout(&self) -> bool;

    /// Disables a stop flag for the interruption checker.
    ///
    /// This method returns `true` if this operation is successful, otherwise `false`.
    fn start(&self) -> bool;

    /// Enables a stop flag for the interruption checker.
    ///
    /// This method returns `true` if this operation is successful, otherwise `false`.
    fn stop(&self) -> bool;

    /// Sets the first search flag.
    ///
    /// If the first search flag is enabled, the timeout and the stop flag are ignored. This method
    /// returns `true` if this operation is successful, otherwise `false`.
    fn set_first(&self, is_first: bool) -> bool;
}

/// A structure of empty interruption checker.
///
/// The empty interruption checker is dummy that ignores all interruptions.
#[derive(Copy, Clone, Debug)]
pub struct EmptyIntrChecker;

impl EmptyIntrChecker
{
    /// Creates an empty interruption checker.
    pub fn new() -> Self
    { EmptyIntrChecker }
}

impl IntrCheck for EmptyIntrChecker
{
    fn check(&self) -> Result<(), Interruption>
    { Ok(()) }

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
