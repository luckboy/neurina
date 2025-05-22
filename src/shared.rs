//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
pub mod converter;
pub mod index_converter;
pub mod intr_check;
pub mod matrix_buffer;
pub mod net;
pub mod network;
pub mod utils;

#[derive(Copy, Clone, Debug)]
pub enum Interruption
{
    Timeout,
    Stop,
    CtrlC,
}
