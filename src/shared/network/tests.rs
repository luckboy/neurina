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
fn test_network_compute_computes_without_panic()
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
    xavier_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100];
    for j in 0usize..100usize {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 1, i_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
    network.compute(&i, 1, 1, |_| (), |_| ());
}

#[test]
fn test_network_compute_computes_without_panic_for_50_columns()
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
    xavier_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100 * 50] ;
    for j in 0usize..(100usize * 50usize) {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 50, i_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
    network.compute(&i, 1, 1, |_| (), |_| ());
}

#[test]
fn test_network_compute_computes_without_panic_for_50_columns_and_depth_and_pv_count()
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
    xavier_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100 * 50] ;
    for j in 0usize..(100usize * 50usize) {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 50, i_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
    network.compute(&i, 4, 3, |_| (), |_| ());
}

#[test]
fn test_network_backpropagate_backpropagates_without_panic()
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
    xavier_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100];
    for j in 0usize..100usize {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 1, i_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
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
    network.compute(&i, 1, 1, |h| hs.push(h), |o| os.push(o));
    network.backpropagate(&i, hs.as_slice(), os.as_slice(), ys.as_slice(), &one);
}

#[test]
fn test_network_backpropagate_backpropagates_without_panic_for_50_columns()
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
    xavier_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100 * 50] ;
    for j in 0usize..(100usize * 50usize) {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 50, i_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
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
    network.compute(&i, 1, 1, |h| hs.push(h), |o| os.push(o));
    network.backpropagate(&i, hs.as_slice(), os.as_slice(), ys.as_slice(), &one);
}

#[test]
fn test_network_backpropagate_backpropagates_without_panic_for_50_columns_and_depth_and_pv_count()
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
    xavier_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let mut i_elems = vec![0.0f32; 100 * 50] ;
    for j in 0usize..(100usize * 50usize) {
        i_elems[j] = random::<f32>() * 2.0 - 1.0;
    }
    let i = Matrix::new_with_elems(100, 50, i_elems.as_slice());
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
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
    network.compute(&i, 4, 3, |h| hs.push(h), |o| os.push(o));
    network.backpropagate(&i, hs.as_slice(), os.as_slice(), ys.as_slice(), &one);
}
