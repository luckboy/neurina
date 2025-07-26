//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Write;
use std::io::stdin;
use std::sync::Arc;
use std::sync::Mutex;
#[cfg(target_family = "unix")]
use libc::SIGINT;
#[cfg(target_family = "unix")]
use libc::SIGTERM;
#[cfg(target_family = "unix")]
use libc::SIG_IGN;
#[cfg(target_family = "unix")]
use libc::signal;
use crate::engine::engine::*;
use crate::engine::engine_id::*;
use crate::engine::io::*;
use crate::engine::print::*;
use crate::engine::uci::*;
use crate::engine::utils::*;
use crate::engine::xboard::*;
use crate::engine::LoopError;
use crate::engine::LoopResult;

/// Performs a loop for a regocnized protocol with the engine identifier.
///
/// See [`protocol_loop`].
pub fn protocol_loop_with_engine_id<F>(stdout_log: Arc<Mutex<StdoutLog>>, engine_id: EngineId, f: F) -> LoopResult<()>
    where F: FnMut(Arc<Mutex<dyn Write + Send + Sync>>, Arc<dyn Print + Send + Sync>) -> LoopResult<Engine>
{
    let mut line = String::new();
    match stdin().read_line(&mut line) {
        Ok(0) => return Ok(()),
        Ok(_) => (),
        Err(err) => return Err(LoopError::Io(err)),
    }
    {
        let mut stdout_log_g = stdout_log.lock().unwrap();
        match stdout_log_g.log_input_line(line.as_str()) {
            Ok(()) => (),
            Err(err) => return Err(LoopError::Io(err)),
        }
    }
    let cmd = str_without_crnl(line.as_str());
    let trimmed_cmd = cmd.trim();
    if trimmed_cmd == "xboard" {
        #[cfg(target_family = "unix")]
        unsafe {
            signal(SIGINT, SIG_IGN);
            signal(SIGTERM, SIG_IGN);
        }
        xboard_loop_with_engine_id(stdout_log, engine_id, f)
    } else if trimmed_cmd == "uci" {
        uci_loop_with_engine_id(stdout_log, engine_id, f)
    } else {
        Err(LoopError::UnrecognizedProtocol)
    }
}

/// Performs a loop for a recognized protocol.
///
/// This function recognizes a protocol and then performs a loop for the recognized protocol. The
/// closure creates an engine for this loop.
pub fn protocol_loop<F>(stdout_log: Arc<Mutex<StdoutLog>>, f: F) -> LoopResult<()>
    where F: FnMut(Arc<Mutex<dyn Write + Send + Sync>>, Arc<dyn Print + Send + Sync>) -> LoopResult<Engine>
{ protocol_loop_with_engine_id(stdout_log, NEURINA_ID, f) }
