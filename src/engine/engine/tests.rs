//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use crate::matrix::Matrix;
use crate::engine::middle_searcher::*;
use crate::engine::neural_searcher::*;
use crate::engine::one_searcher::*;
use crate::engine::simple_eval_fun::*;
use crate::engine::thinker::*;
use crate::shared::converter::*;
use crate::shared::index_converter::*;
use crate::shared::intr_check::*;
use crate::shared::network::*;
use crate::shared::xavier_init::*;
use super::*;

#[test]
fn test_engine_go_thinks_without_panic()
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
    let eval_fun = Arc::new(SimpleEvalFun::new());
    let neural_searcher = Arc::new(NeuralSearcher::new(intr_checker, converter, network));
    let middle_searcher = MiddleSearcher::new(eval_fun, neural_searcher);
    let one_searcher = Arc::new(OneSearcher::new(middle_searcher, 2));
    let mut new_move_chain = MoveChain::new_initial();
    new_move_chain.push_uci_list("e2e4 e7e5").unwrap();
    let cursor = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
    let printer = Arc::new(EmptyPrinter::new());
    let thinker = Arc::new(Thinker::new(one_searcher, cursor, printer, Arc::new(Mutex::new(None))));
    let engine = Engine::new(thinker);
    engine.do_move_chain(|move_chain| {
            *move_chain = new_move_chain;
    });
    engine.go(None, Some(5), None, None, false, true, true, true);
    engine.quit();
    engine.join_thread();
}
