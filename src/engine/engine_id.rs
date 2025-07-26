//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// A structure of engine identifier.
///
/// The engine identifier contains an egine name and engine author(s).
#[derive(Copy, Clone, Debug)]
pub struct EngineId
{
    /// The engine name.
    pub name: &'static str,
    /// The first engine author(s).
    pub first_author: Option<&'static str>,
    /// The last engine author(s).
    pub last_author: Option<&'static str>,
}

/// A neurina engine identifier.
pub const NEURINA_ID: EngineId = EngineId {
    name: concat!("Neurina ", env!("CARGO_PKG_VERSION")),
    first_author: None,
    last_author: None,
};
