//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use rand::random;

pub fn xavier_init(elems: &mut [f32], input_count: usize, output_count: usize)
{
    let u = (6.0 / ((input_count as f32) + (output_count as f32))).sqrt();
    for i in 0..elems.len() {
        elems[i] = (random::<f32>() * 2.0 - 1.0) * u;
    }
}
