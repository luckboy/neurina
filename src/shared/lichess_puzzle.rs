//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::serde::Deserialize;
use crate::serde::Serialize;

/// A structure of lichess puzzle.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct LichessPuzzle
{
    pub PuzzleId: String,
    pub FEN: String,
    pub Moves: String,
    pub Rating: String,
    pub RatingDeviation: String,
    pub Popularity: String,
    pub NbPlays: String,
    pub Themes: String,
    pub GameUrl: String,
    pub OpeningTags: String,
}
