//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
pub fn str_without_nl(s: &str) -> &str
{
    if s.ends_with('\n') {
        &s[0..(s.len() - 1)]
    } else {
        s
    }
}

pub fn str_without_crnl(s: &str) -> &str
{
    if s.ends_with('\n') {
        let s2 = &s[0..(s.len() - 1)];
        if s2.ends_with('\r') {
            &s2[0..(s2.len() - 1)]
        } else {
            s2
        }
    } else {
        s
    }
}
