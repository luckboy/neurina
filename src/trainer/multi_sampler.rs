//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::trainer::data_sample::*;
use crate::trainer::sample::*;

#[derive(Copy, Clone, Debug)]
pub struct MultiSampler;

impl MultiSampler
{
    pub fn new() -> Self
    { MultiSampler }
}

impl Sample for MultiSampler
{
    fn samples(&self, sample: &DataSample) -> Option<Vec<DataSample>>
    {
        let mut samples: Vec<DataSample> = Vec::new();
        let mut tmp_board = sample.board.clone();
        samples.push(DataSample::new(tmp_board.clone(), sample.moves.clone()));
        for (i, mv) in sample.moves.iter().enumerate() {
            match tmp_board.make_move(*mv) {
                Ok(tmp_new_board) => {
                    tmp_board = tmp_new_board;
                    if !sample.moves[(i + 1)..].is_empty() {
                        samples.push(DataSample::new(tmp_board.clone(), sample.moves[(i + 1)..].to_vec()));
                    }
                },
                Err(_) => return None,
            }
        }
        Some(samples)
    }
}
