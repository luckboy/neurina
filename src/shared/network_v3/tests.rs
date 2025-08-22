//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;
use crate::shared::xavier_init::*;
use rand::random;
use super::*;

#[test]
fn test_network_v3_compute_computes_without_panic()
{
    let mut iw_elems = vec![0.0f32; 200 * 100];
    xavier_init(iw_elems.as_mut_slice(), 100, 200);
    let iw = Matrix::new_with_elems(200, 100, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 200];
    xavier_init(ib_elems.as_mut_slice(), 100, 200);
    let ib = Matrix::new_with_elems(200, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 200 * 200];
    xavier_init(sw_elems.as_mut_slice(), 200, 200);
    let sw = Matrix::new_with_elems(200, 200, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 200];
    xavier_init(sb_elems.as_mut_slice(), 200, 200);
    let sb = Matrix::new_with_elems(200, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 200 * 200];
    xavier_init(pw_elems.as_mut_slice(), 200, 200);
    let pw = Matrix::new_with_elems(200, 200, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 200];
    xavier_init(pb_elems.as_mut_slice(), 200, 200);
    let pb = Matrix::new_with_elems(200, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; 150 * 200];
    xavier_sqrt_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_sqrt_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100];
    for j in 0usize..100usize {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 1, i_elems.as_slice());
    let network = NetworkV3::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let mut hs: Vec<Matrix> = Vec::new();
    let mut os: Vec<Matrix> = Vec::new();
    network.compute(&i, 1, 1, |h| {
            hs.push(h);
            Ok(())
    }, |o| {
            os.push(o);
            Ok(())
    }).unwrap();
    assert_eq!(3, hs.len());
    assert_eq!(200, hs[0].row_count());
    assert_eq!(1, hs[0].col_count());
    assert_eq!(200, hs[1].row_count());
    assert_eq!(1, hs[1].col_count());
    assert_eq!(200, hs[2].row_count());
    assert_eq!(1, hs[2].col_count());
    assert_eq!(1, os.len());
    assert_eq!(150, os[0].row_count());
    assert_eq!(1, os[0].col_count());
}

#[test]
fn test_network_v3_compute_computes_without_panic_for_50_columns()
{
    let mut iw_elems = vec![0.0f32; 200 * 100];
    xavier_init(iw_elems.as_mut_slice(), 100, 200);
    let iw = Matrix::new_with_elems(200, 100, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 200];
    xavier_init(ib_elems.as_mut_slice(), 100, 200);
    let ib = Matrix::new_with_elems(200, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 200 * 200];
    xavier_init(sw_elems.as_mut_slice(), 200, 200);
    let sw = Matrix::new_with_elems(200, 200, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 200];
    xavier_init(sb_elems.as_mut_slice(), 200, 200);
    let sb = Matrix::new_with_elems(200, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 200 * 200];
    xavier_init(pw_elems.as_mut_slice(), 200, 200);
    let pw = Matrix::new_with_elems(200, 200, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 200];
    xavier_init(pb_elems.as_mut_slice(), 200, 200);
    let pb = Matrix::new_with_elems(200, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; 150 * 200];
    xavier_sqrt_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_sqrt_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100 * 50] ;
    for j in 0usize..(100usize * 50usize) {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 50, i_elems.as_slice());
    let network = NetworkV3::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let mut hs: Vec<Matrix> = Vec::new();
    let mut os: Vec<Matrix> = Vec::new();
    network.compute(&i, 1, 1, |h| {
            hs.push(h);
            Ok(())
    }, |o| {
            os.push(o);
            Ok(())
    }).unwrap();
    assert_eq!(3, hs.len());
    assert_eq!(200, hs[0].row_count());
    assert_eq!(50, hs[0].col_count());
    assert_eq!(200, hs[1].row_count());
    assert_eq!(50, hs[1].col_count());
    assert_eq!(200, hs[2].row_count());
    assert_eq!(50, hs[2].col_count());
    assert_eq!(1, os.len());
    assert_eq!(150, os[0].row_count());
    assert_eq!(50, os[0].col_count());
}

#[test]
fn test_network_v3_compute_computes_without_panic_for_50_columns_and_depth_and_pv_count()
{
    let mut iw_elems = vec![0.0f32; 200 * 100];
    xavier_init(iw_elems.as_mut_slice(), 100, 200);
    let iw = Matrix::new_with_elems(200, 100, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 200];
    xavier_init(ib_elems.as_mut_slice(), 100, 200);
    let ib = Matrix::new_with_elems(200, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 200 * 200];
    xavier_init(sw_elems.as_mut_slice(), 200, 200);
    let sw = Matrix::new_with_elems(200, 200, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 200];
    xavier_init(sb_elems.as_mut_slice(), 200, 200);
    let sb = Matrix::new_with_elems(200, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 200 * 200];
    xavier_init(pw_elems.as_mut_slice(), 200, 200);
    let pw = Matrix::new_with_elems(200, 200, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 200];
    xavier_init(pb_elems.as_mut_slice(), 200, 200);
    let pb = Matrix::new_with_elems(200, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; 150 * 200];
    xavier_sqrt_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_sqrt_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100 * 50] ;
    for j in 0usize..(100usize * 50usize) {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 50, i_elems.as_slice());
    let network = NetworkV3::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let mut hs: Vec<Matrix> = Vec::new();
    let mut os: Vec<Matrix> = Vec::new();
    network.compute(&i, 4, 3, |h| {
            hs.push(h);
            Ok(())
    }, |o| {
            os.push(o);
            Ok(())
    }).unwrap();
    assert_eq!(1 + 4 + 3, hs.len());
    assert_eq!(200, hs[0].row_count());
    assert_eq!(50, hs[0].col_count());
    assert_eq!(200, hs[1].row_count());
    assert_eq!(50, hs[1].col_count());
    assert_eq!(200, hs[2].row_count());
    assert_eq!(50, hs[2].col_count());
    assert_eq!(200, hs[3].row_count());
    assert_eq!(50, hs[3].col_count());
    assert_eq!(200, hs[4].row_count());
    assert_eq!(50, hs[4].col_count());
    assert_eq!(200, hs[5].row_count());
    assert_eq!(50, hs[5].col_count());
    assert_eq!(200, hs[6].row_count());
    assert_eq!(50, hs[6].col_count());
    assert_eq!(200, hs[7].row_count());
    assert_eq!(50, hs[7].col_count());
    assert_eq!(3, os.len());
    assert_eq!(150, os[0].row_count());
    assert_eq!(50, os[0].col_count());
    assert_eq!(150, os[1].row_count());
    assert_eq!(50, os[1].col_count());
    assert_eq!(150, os[2].row_count());
    assert_eq!(50, os[2].col_count());
}

#[test]
fn test_network_v3_backpropagate_backpropagates_without_panic()
{
    let mut iw_elems = vec![0.0f32; 200 * 100];
    xavier_init(iw_elems.as_mut_slice(), 100, 200);
    let iw = Matrix::new_with_elems(200, 100, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 200];
    xavier_init(ib_elems.as_mut_slice(), 100, 200);
    let ib = Matrix::new_with_elems(200, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 200 * 200];
    xavier_init(sw_elems.as_mut_slice(), 200, 200);
    let sw = Matrix::new_with_elems(200, 200, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 200];
    xavier_init(sb_elems.as_mut_slice(), 200, 200);
    let sb = Matrix::new_with_elems(200, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 200 * 200];
    xavier_init(pw_elems.as_mut_slice(), 200, 200);
    let pw = Matrix::new_with_elems(200, 200, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 200];
    xavier_init(pb_elems.as_mut_slice(), 200, 200);
    let pb = Matrix::new_with_elems(200, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; 150 * 200];
    xavier_sqrt_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_sqrt_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100];
    for j in 0usize..100usize {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 1, i_elems.as_slice());
    let network = NetworkV3::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let one_elems = vec![1.0f32; 1];
    let one = Matrix::new_with_elems(1, 1, one_elems.as_slice());
    let mut hs: Vec<Matrix> = Vec::new();
    let mut os: Vec<Matrix> = Vec::new();
    let mut ys: Vec<Matrix> = Vec::new();
    for _ in 0usize..1usize {
        let mut y_elems = vec![0.0f32; 150]; 
        for j in 0usize..1usize {
            y_elems[((random::<u32>() % 150) as usize) * 1 + j] = 1.0;
        }
        let y = Matrix::new_with_elems(150, 1, y_elems.as_slice());
        ys.push(y);
    }
    network.compute(&i, 1, 1, |h| {
            hs.push(h);
            Ok(())
    }, |o| {
            os.push(o);
            Ok(())
    }).unwrap();
    let dj_dnet = network.backpropagate(&i, hs.as_slice(), os.as_slice(), ys.as_slice(), &one);
    assert_eq!(200, dj_dnet.iw().row_count());
    assert_eq!(100, dj_dnet.iw().col_count());
    assert_eq!(200, dj_dnet.ib().row_count());
    assert_eq!(1, dj_dnet.ib().col_count());
    assert_eq!(200, dj_dnet.sw().row_count());
    assert_eq!(200, dj_dnet.sw().col_count());
    assert_eq!(200, dj_dnet.sb().row_count());
    assert_eq!(1, dj_dnet.sb().col_count());
    assert_eq!(200, dj_dnet.pw().row_count());
    assert_eq!(200, dj_dnet.pw().col_count());
    assert_eq!(200, dj_dnet.pb().row_count());
    assert_eq!(1, dj_dnet.pb().col_count());
    assert_eq!(150, dj_dnet.ow().row_count());
    assert_eq!(200, dj_dnet.ow().col_count());
    assert_eq!(150, dj_dnet.ob().row_count());
    assert_eq!(1, dj_dnet.ob().col_count());
}

#[test]
fn test_network_v3_backpropagate_backpropagates_without_panic_for_50_columns()
{
    let mut iw_elems = vec![0.0f32; 200 * 100];
    xavier_init(iw_elems.as_mut_slice(), 100, 200);
    let iw = Matrix::new_with_elems(200, 100, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 200];
    xavier_init(ib_elems.as_mut_slice(), 100, 200);
    let ib = Matrix::new_with_elems(200, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 200 * 200];
    xavier_init(sw_elems.as_mut_slice(), 200, 200);
    let sw = Matrix::new_with_elems(200, 200, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 200];
    xavier_init(sb_elems.as_mut_slice(), 200, 200);
    let sb = Matrix::new_with_elems(200, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 200 * 200];
    xavier_init(pw_elems.as_mut_slice(), 200, 200);
    let pw = Matrix::new_with_elems(200, 200, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 200];
    xavier_init(pb_elems.as_mut_slice(), 200, 200);
    let pb = Matrix::new_with_elems(200, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; 150 * 200];
    xavier_sqrt_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_sqrt_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100 * 50] ;
    for j in 0usize..(100usize * 50usize) {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 50, i_elems.as_slice());
    let network = NetworkV3::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let one_elems = vec![1.0f32; 50];
    let one = Matrix::new_with_elems(50, 1, one_elems.as_slice());
    let mut hs: Vec<Matrix> = Vec::new();
    let mut os: Vec<Matrix> = Vec::new();
    let mut ys: Vec<Matrix> = Vec::new();
    for _ in 0usize..1usize {
        let mut y_elems = vec![0.0f32; 150 * 50]; 
        for j in 0usize..50usize {
            y_elems[((random::<u32>() % 150) as usize) * 50 + j] = 1.0;
        }
        let y = Matrix::new_with_elems(150, 50, y_elems.as_slice());
        ys.push(y);
    }
    network.compute(&i, 1, 1, |h| {
            hs.push(h);
            Ok(())
    }, |o| {
            os.push(o);
            Ok(())
    }).unwrap();
    let dj_dnet = network.backpropagate(&i, hs.as_slice(), os.as_slice(), ys.as_slice(), &one);
    assert_eq!(200, dj_dnet.iw().row_count());
    assert_eq!(100, dj_dnet.iw().col_count());
    assert_eq!(200, dj_dnet.ib().row_count());
    assert_eq!(1, dj_dnet.ib().col_count());
    assert_eq!(200, dj_dnet.sw().row_count());
    assert_eq!(200, dj_dnet.sw().col_count());
    assert_eq!(200, dj_dnet.sb().row_count());
    assert_eq!(1, dj_dnet.sb().col_count());
    assert_eq!(200, dj_dnet.pw().row_count());
    assert_eq!(200, dj_dnet.pw().col_count());
    assert_eq!(200, dj_dnet.pb().row_count());
    assert_eq!(1, dj_dnet.pb().col_count());
    assert_eq!(150, dj_dnet.ow().row_count());
    assert_eq!(200, dj_dnet.ow().col_count());
    assert_eq!(150, dj_dnet.ob().row_count());
    assert_eq!(1, dj_dnet.ob().col_count());
}

#[test]
fn test_network_v3_backpropagate_backpropagates_without_panic_for_50_columns_and_depth_and_pv_count()
{
    let mut iw_elems = vec![0.0f32; 200 * 100];
    xavier_init(iw_elems.as_mut_slice(), 100, 200);
    let iw = Matrix::new_with_elems(200, 100, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 200];
    xavier_init(ib_elems.as_mut_slice(), 100, 200);
    let ib = Matrix::new_with_elems(200, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 200 * 200];
    xavier_init(sw_elems.as_mut_slice(), 200, 200);
    let sw = Matrix::new_with_elems(200, 200, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 200];
    xavier_init(sb_elems.as_mut_slice(), 200, 200);
    let sb = Matrix::new_with_elems(200, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 200 * 200];
    xavier_init(pw_elems.as_mut_slice(), 200, 200);
    let pw = Matrix::new_with_elems(200, 200, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 200];
    xavier_init(pb_elems.as_mut_slice(), 200, 200);
    let pb = Matrix::new_with_elems(200, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; 150 * 200];
    xavier_sqrt_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_sqrt_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100 * 50] ;
    for j in 0usize..(100usize * 50usize) {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 50, i_elems.as_slice());
    let network = NetworkV3::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let one_elems = vec![1.0f32; 50];
    let one = Matrix::new_with_elems(50, 1, one_elems.as_slice());
    let mut hs: Vec<Matrix> = Vec::new();
    let mut os: Vec<Matrix> = Vec::new();
    let mut ys: Vec<Matrix> = Vec::new();
    for _ in 0usize..3usize {
        let mut y_elems = vec![0.0f32; 150 * 50]; 
        for j in 0usize..50usize {
            y_elems[((random::<u32>() % 150) as usize) * 50 + j] = 1.0;
        }
        let y = Matrix::new_with_elems(150, 50, y_elems.as_slice());
        ys.push(y);
    }
    network.compute(&i, 4, 3, |h| {
            hs.push(h);
            Ok(())
    }, |o| {
            os.push(o);
            Ok(())
    }).unwrap();
    let dj_dnet = network.backpropagate(&i, hs.as_slice(), os.as_slice(), ys.as_slice(), &one);
    assert_eq!(200, dj_dnet.iw().row_count());
    assert_eq!(100, dj_dnet.iw().col_count());
    assert_eq!(200, dj_dnet.ib().row_count());
    assert_eq!(1, dj_dnet.ib().col_count());
    assert_eq!(200, dj_dnet.sw().row_count());
    assert_eq!(200, dj_dnet.sw().col_count());
    assert_eq!(200, dj_dnet.sb().row_count());
    assert_eq!(1, dj_dnet.sb().col_count());
    assert_eq!(200, dj_dnet.pw().row_count());
    assert_eq!(200, dj_dnet.pw().col_count());
    assert_eq!(200, dj_dnet.pb().row_count());
    assert_eq!(1, dj_dnet.pb().col_count());
    assert_eq!(150, dj_dnet.ow().row_count());
    assert_eq!(200, dj_dnet.ow().col_count());
    assert_eq!(150, dj_dnet.ob().row_count());
    assert_eq!(1, dj_dnet.ob().col_count());
}

#[test]
fn test_network_v3_check_returns_true()
{
    let mut iw_elems = vec![0.0f32; 200 * 100];
    xavier_init(iw_elems.as_mut_slice(), 100, 200);
    let iw = Matrix::new_with_elems(200, 100, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 200];
    xavier_init(ib_elems.as_mut_slice(), 100, 200);
    let ib = Matrix::new_with_elems(200, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 200 * 200];
    xavier_init(sw_elems.as_mut_slice(), 200, 200);
    let sw = Matrix::new_with_elems(200, 200, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 200];
    xavier_init(sb_elems.as_mut_slice(), 200, 200);
    let sb = Matrix::new_with_elems(200, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 200 * 200];
    xavier_init(pw_elems.as_mut_slice(), 200, 200);
    let pw = Matrix::new_with_elems(200, 200, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 200];
    xavier_init(pb_elems.as_mut_slice(), 200, 200);
    let pb = Matrix::new_with_elems(200, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; 150 * 200];
    xavier_sqrt_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_sqrt_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let network = NetworkV3::new(iw, ib, sw, sb, pw, pb, ow, ob);
    assert_eq!(true, network.check(100, 150));
}

#[test]
fn test_network_v3_check_returns_false()
{
    let mut iw_elems = vec![0.0f32; 200 * 100];
    xavier_init(iw_elems.as_mut_slice(), 100, 200);
    let iw = Matrix::new_with_elems(200, 100, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 200];
    xavier_init(ib_elems.as_mut_slice(), 100, 200);
    let ib = Matrix::new_with_elems(200, 1, ib_elems.as_slice());
    let mut sw_elems = vec![0.0f32; 200 * 201];
    xavier_init(sw_elems.as_mut_slice(), 200, 200);
    let sw = Matrix::new_with_elems(200, 201, sw_elems.as_slice());
    let mut sb_elems = vec![0.0f32; 200];
    xavier_init(sb_elems.as_mut_slice(), 200, 200);
    let sb = Matrix::new_with_elems(200, 1, sb_elems.as_slice());
    let mut pw_elems = vec![0.0f32; 200 * 200];
    xavier_init(pw_elems.as_mut_slice(), 200, 200);
    let pw = Matrix::new_with_elems(200, 200, pw_elems.as_slice());
    let mut pb_elems = vec![0.0f32; 200];
    xavier_init(pb_elems.as_mut_slice(), 200, 200);
    let pb = Matrix::new_with_elems(200, 1, pb_elems.as_slice());
    let mut ow_elems = vec![0.0f32; 150 * 200];
    xavier_sqrt_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_sqrt_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let network = NetworkV3::new(iw, ib, sw, sb, pw, pb, ow, ob);
    assert_eq!(false, network.check(100, 150));
}
