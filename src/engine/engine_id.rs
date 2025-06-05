//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
#[derive(Copy, Clone, Debug)]
pub struct EngineId
{
    pub name: &'static str,
    pub first_author: Option<&'static str>,
    pub last_author: Option<&'static str>,
}

pub const NEURINA_ID: EngineId = EngineId {
    name: concat!("Neurina ", env!("CARGO_PKG_VERSION")),
    first_author: None,
    last_author: None,
};
