//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use super::*;

#[test]
fn test_selector_select_selects_puzzles_without_panic()
{
    let intr_checker = Arc::new(EmptyIntrChecker::new());
    let cursor = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
    let printer = Arc::new(EmptyPrinter::new());
    let selector = Selector::new(intr_checker, cursor, printer);
    // Sample of puzzles is from https://database.lichess.org.
    let s = "
PuzzleId,FEN,Moves,Rating,RatingDeviation,Popularity,NbPlays,Themes,GameUrl,OpeningTags
00sHx,q3k1nr/1pp1nQpp/3p4/1P2p3/4P3/B1PP1b2/B5PP/5K2 b k - 0 17,e8d7 a2e6 d7d8 f7f8,1760,80,83,72,mate mateIn2 middlegame short,https://lichess.org/yyznGmXs/black#34,Italian_Game Italian_Game_Classical_Variation
00sJ9,r3r1k1/p4ppp/2p2n2/1p6/3P1qb1/2NQR3/PPB2PP1/R1B3K1 w - - 5 18,e3g3 e8e1 g1h2 e1c1 a1c1 f4h6 h2g1 h6c1,2671,105,87,325,advantage attraction fork middlegame sacrifice veryLong,https://lichess.org/gyFeQsOE#35,French_Defense French_Defense_Exchange_Variation
00sJb,Q1b2r1k/p2np2p/5bp1/q7/5P2/4B3/PPP3PP/2KR1B1R w - - 1 17,d1d7 a5e1 d7d1 e1e3 c1b1 e3b6,2235,76,97,64,advantage fork long,https://lichess.org/kiuvTFoE#33,Sicilian_Defense Sicilian_Defense_Dragon_Variation
00sO1,1k1r4/pp3pp1/2p1p3/4b3/P3n1P1/8/KPP2PN1/3rBR1R b - - 2 31,b8c7 e1a5 b7b6 f1d1,998,85,94,293,advantage discoveredAttack master middlegame short,https://lichess.org/vsfFkG0s/black#62,
";
    let s2 = &s[1..];
    let cursor2 = Cursor::new(s2);
    let mut reader = LichessPuzzleReader::from_reader(cursor2);
    let mut puzzles = reader.puzzles(None);
    let mut cursor3 = Cursor::new(Vec::<u8>::new());
    match selector.select(&mut puzzles, &mut cursor3, 2) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_selector_select_selects_puzzles_without_panic_for_divider_that_is_three()
{
    let intr_checker = Arc::new(EmptyIntrChecker::new());
    let cursor = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
    let printer = Arc::new(EmptyPrinter::new());
    let selector = Selector::new(intr_checker, cursor, printer);
    // Sample of puzzles is from https://database.lichess.org.
    let s = "
PuzzleId,FEN,Moves,Rating,RatingDeviation,Popularity,NbPlays,Themes,GameUrl,OpeningTags
00sHx,q3k1nr/1pp1nQpp/3p4/1P2p3/4P3/B1PP1b2/B5PP/5K2 b k - 0 17,e8d7 a2e6 d7d8 f7f8,1760,80,83,72,mate mateIn2 middlegame short,https://lichess.org/yyznGmXs/black#34,Italian_Game Italian_Game_Classical_Variation
00sJ9,r3r1k1/p4ppp/2p2n2/1p6/3P1qb1/2NQR3/PPB2PP1/R1B3K1 w - - 5 18,e3g3 e8e1 g1h2 e1c1 a1c1 f4h6 h2g1 h6c1,2671,105,87,325,advantage attraction fork middlegame sacrifice veryLong,https://lichess.org/gyFeQsOE#35,French_Defense French_Defense_Exchange_Variation
00sJb,Q1b2r1k/p2np2p/5bp1/q7/5P2/4B3/PPP3PP/2KR1B1R w - - 1 17,d1d7 a5e1 d7d1 e1e3 c1b1 e3b6,2235,76,97,64,advantage fork long,https://lichess.org/kiuvTFoE#33,Sicilian_Defense Sicilian_Defense_Dragon_Variation
00sO1,1k1r4/pp3pp1/2p1p3/4b3/P3n1P1/8/KPP2PN1/3rBR1R b - - 2 31,b8c7 e1a5 b7b6 f1d1,998,85,94,293,advantage discoveredAttack master middlegame short,https://lichess.org/vsfFkG0s/black#62,
";
    let s2 = &s[1..];
    let cursor2 = Cursor::new(s2);
    let mut reader = LichessPuzzleReader::from_reader(cursor2);
    let mut puzzles = reader.puzzles(None);
    let mut cursor3 = Cursor::new(Vec::<u8>::new());
    match selector.select(&mut puzzles, &mut cursor3, 3) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}
