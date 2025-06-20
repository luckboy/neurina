//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use crate::matrix::Matrix;
use crate::shared::network::*;
use crate::shared::converter::*;
use crate::shared::index_converter::*;
use crate::shared::intr_check::*;
use crate::shared::xavier_init::*;
use crate::trainer::algorithms::gd::*;
use crate::trainer::gradient_adder::*;
use crate::trainer::lichess_puzzles::*;
use crate::trainer::multi_sampler::*;
use crate::trainer::single_sampler::*;
use crate::trainer::print::*;
use super::*;

#[test]
fn test_trainer_do_epoch_trains_without_panic_with_single_sampler_and_gradient_adder()
{
    let converter = Converter::new(IndexConverter::new());
    let mut iw_elems = vec![0.0f32; 256 * Converter::BOARD_ROW_COUNT];
    xavier_init(iw_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, 256);
    let iw = Matrix::new_with_elems(256, Converter::BOARD_ROW_COUNT, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 256];
    xavier_init(ib_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, 256);
    let ib = Matrix::new_with_elems(256, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 256 * 256];
    xavier_init(sw_elems.as_mut_slice(), 256, 256);
    let sw = Matrix::new_with_elems(256, 256, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 256];
    xavier_init(sb_elems.as_mut_slice(), 256, 256);
    let sb = Matrix::new_with_elems(256, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 256 * 256];
    xavier_init(pw_elems.as_mut_slice(), 256, 256);
    let pw = Matrix::new_with_elems(256, 256, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 256];
    xavier_init(pb_elems.as_mut_slice(), 256, 256);
    let pb = Matrix::new_with_elems(256, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; converter.move_row_count() * 256];
    xavier_init(ow_elems.as_mut_slice(), 256, converter.move_row_count());
    let ow = Matrix::new_with_elems(converter.move_row_count(), 256, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; converter.move_row_count()];
    xavier_init(ob_elems.as_mut_slice(), 256, converter.move_row_count());
    let ob = Matrix::new_with_elems(converter.move_row_count(), 1, ob_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let intr_checker = Arc::new(EmptyIntrChecker::new());
    let sampler = Arc::new(SingleSampler::new());
    let gradient_adder = GradientAdder::new_with_max_col_count(intr_checker, converter, network, 3);
    let params = GdParams { eta: 0.1, };
    let state = GdState { epoch: 1, };
    let alg = Arc::new(GdAlg::new(gradient_adder, params, state));
    let cursor = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
    let printer = Arc::new(EmptyPrinter::new());
    let trainer = Trainer::new(sampler, alg, cursor, printer);
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
    match trainer.do_epoch(&mut puzzles) {
        Ok((_, _, err_count)) => assert_eq!(0, err_count),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_trainer_do_epoch_trains_without_panic_with_multi_sampler_and_gradient_adder()
{
    let converter = Converter::new(IndexConverter::new());
    let mut iw_elems = vec![0.0f32; 256 * Converter::BOARD_ROW_COUNT];
    xavier_init(iw_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, 256);
    let iw = Matrix::new_with_elems(256, Converter::BOARD_ROW_COUNT, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 256];
    xavier_init(ib_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, 256);
    let ib = Matrix::new_with_elems(256, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 256 * 256];
    xavier_init(sw_elems.as_mut_slice(), 256, 256);
    let sw = Matrix::new_with_elems(256, 256, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 256];
    xavier_init(sb_elems.as_mut_slice(), 256, 256);
    let sb = Matrix::new_with_elems(256, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 256 * 256];
    xavier_init(pw_elems.as_mut_slice(), 256, 256);
    let pw = Matrix::new_with_elems(256, 256, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 256];
    xavier_init(pb_elems.as_mut_slice(), 256, 256);
    let pb = Matrix::new_with_elems(256, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; converter.move_row_count() * 256];
    xavier_init(ow_elems.as_mut_slice(), 256, converter.move_row_count());
    let ow = Matrix::new_with_elems(converter.move_row_count(), 256, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; converter.move_row_count()];
    xavier_init(ob_elems.as_mut_slice(), 256, converter.move_row_count());
    let ob = Matrix::new_with_elems(converter.move_row_count(), 1, ob_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let intr_checker = Arc::new(EmptyIntrChecker::new());
    let sampler = Arc::new(MultiSampler::new());
    let gradient_adder = GradientAdder::new_with_max_col_count(intr_checker, converter, network, 3);
    let params = GdParams { eta: 0.1, };
    let state = GdState { epoch: 1, };
    let alg = Arc::new(GdAlg::new(gradient_adder, params, state));
    let cursor = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
    let printer = Arc::new(EmptyPrinter::new());
    let trainer = Trainer::new(sampler, alg, cursor, printer);
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
    match trainer.do_epoch(&mut puzzles) {
        Ok((_, _, err_count)) => assert_eq!(0, err_count),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_trainer_do_result_computes_result_without_panic_with_single_sampler_and_gradient_adder()
{
    let converter = Converter::new(IndexConverter::new());
    let mut iw_elems = vec![0.0f32; 256 * Converter::BOARD_ROW_COUNT];
    xavier_init(iw_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, 256);
    let iw = Matrix::new_with_elems(256, Converter::BOARD_ROW_COUNT, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 256];
    xavier_init(ib_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, 256);
    let ib = Matrix::new_with_elems(256, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 256 * 256];
    xavier_init(sw_elems.as_mut_slice(), 256, 256);
    let sw = Matrix::new_with_elems(256, 256, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 256];
    xavier_init(sb_elems.as_mut_slice(), 256, 256);
    let sb = Matrix::new_with_elems(256, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 256 * 256];
    xavier_init(pw_elems.as_mut_slice(), 256, 256);
    let pw = Matrix::new_with_elems(256, 256, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 256];
    xavier_init(pb_elems.as_mut_slice(), 256, 256);
    let pb = Matrix::new_with_elems(256, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; converter.move_row_count() * 256];
    xavier_init(ow_elems.as_mut_slice(), 256, converter.move_row_count());
    let ow = Matrix::new_with_elems(converter.move_row_count(), 256, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; converter.move_row_count()];
    xavier_init(ob_elems.as_mut_slice(), 256, converter.move_row_count());
    let ob = Matrix::new_with_elems(converter.move_row_count(), 1, ob_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let intr_checker = Arc::new(EmptyIntrChecker::new());
    let sampler = Arc::new(SingleSampler::new());
    let gradient_adder = GradientAdder::new_with_max_col_count(intr_checker, converter, network, 3);
    let params = GdParams { eta: 0.1, };
    let state = GdState { epoch: 1, };
    let alg = Arc::new(GdAlg::new(gradient_adder, params, state));
    let cursor = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
    let printer = Arc::new(EmptyPrinter::new());
    let trainer = Trainer::new(sampler, alg, cursor, printer);
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
    match trainer.do_result(&mut puzzles) {
        Ok((_, _, err_count)) => assert_eq!(0, err_count),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_trainer_do_result_computes_result_without_panic_with_multi_sampler_and_gradient_adder()
{
    let converter = Converter::new(IndexConverter::new());
    let mut iw_elems = vec![0.0f32; 256 * Converter::BOARD_ROW_COUNT];
    xavier_init(iw_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, 256);
    let iw = Matrix::new_with_elems(256, Converter::BOARD_ROW_COUNT, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 256];
    xavier_init(ib_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, 256);
    let ib = Matrix::new_with_elems(256, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 256 * 256];
    xavier_init(sw_elems.as_mut_slice(), 256, 256);
    let sw = Matrix::new_with_elems(256, 256, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 256];
    xavier_init(sb_elems.as_mut_slice(), 256, 256);
    let sb = Matrix::new_with_elems(256, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 256 * 256];
    xavier_init(pw_elems.as_mut_slice(), 256, 256);
    let pw = Matrix::new_with_elems(256, 256, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 256];
    xavier_init(pb_elems.as_mut_slice(), 256, 256);
    let pb = Matrix::new_with_elems(256, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; converter.move_row_count() * 256];
    xavier_init(ow_elems.as_mut_slice(), 256, converter.move_row_count());
    let ow = Matrix::new_with_elems(converter.move_row_count(), 256, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; converter.move_row_count()];
    xavier_init(ob_elems.as_mut_slice(), 256, converter.move_row_count());
    let ob = Matrix::new_with_elems(converter.move_row_count(), 1, ob_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let intr_checker = Arc::new(EmptyIntrChecker::new());
    let sampler = Arc::new(MultiSampler::new());
    let gradient_adder = GradientAdder::new_with_max_col_count(intr_checker, converter, network, 3);
    let params = GdParams { eta: 0.1, };
    let state = GdState { epoch: 1, };
    let alg = Arc::new(GdAlg::new(gradient_adder, params, state));
    let cursor = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
    let printer = Arc::new(EmptyPrinter::new());
    let trainer = Trainer::new(sampler, alg, cursor, printer);
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
    match trainer.do_result(&mut puzzles) {
        Ok((_, _, err_count)) => assert_eq!(0, err_count),
        Err(_) => assert!(false),
    }
}
