//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
pub(crate) mod backend;
pub(crate) mod config;
pub(crate) mod converter;
pub(crate) mod ctrl_c_intr_checker;
pub(crate) mod index_converter;
pub(crate) mod intr_check;
pub(crate) mod io;
pub(crate) mod lichess_puzzle;
pub(crate) mod matrix_buffer;
pub(crate) mod net;
pub(crate) mod network;
pub(crate) mod private;
pub(crate) mod utils;
pub(crate) mod xavier_init;

pub use backend::*;
pub use config::*;
pub use converter::*;
pub use ctrl_c_intr_checker::*;
pub use index_converter::*;
pub use intr_check::*;
pub use io::*;
pub use lichess_puzzle::*;
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
