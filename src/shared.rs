//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
pub(crate) mod converter;
pub(crate) mod index_converter;
pub(crate) mod intr_check;
pub(crate) mod matrix_buffer;
pub(crate) mod net;
pub(crate) mod network;
pub(crate) mod utils;
pub(crate) mod xavier_init;

pub use converter::*;
pub use index_converter::*;
pub use intr_check::*;
pub use matrix_buffer::*;
pub use net::*;
pub use network::*;
pub use utils::*;
pub use xavier_init::*;

#[derive(Copy, Clone, Debug)]
pub enum Interruption
{
    Timeout,
    Stop,
    CtrlC,
}
