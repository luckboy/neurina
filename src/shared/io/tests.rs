//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use crate::matrix::Matrix;
use crate::shared::xavier_init::*;
use super::*;

#[test]
fn test_write_matrix_and_read_matrix_writes_matrix_and_reads_matrix()
{
    let mut a_elems = vec![0.0f32; 200 * 100];
    xavier_init(a_elems.as_mut_slice(), 100, 200);
    let a = Matrix::new_with_elems(200, 100, a_elems.as_slice());
    let mut cursor = Cursor::new(Vec::<u8>::new());
    match write_matrix(&mut cursor, &a) {
        Ok(()) => {
            cursor.set_position(0);
            match read_matrix(&mut cursor) {
                Ok(b) => {
                    assert_eq!(a.row_count(), b.row_count());
                    assert_eq!(a.col_count(), b.col_count());
                    assert_eq!(b.elems(), a.elems());
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_network_and_read_network_writes_network_and_reads_network()
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
    let network = Network::new(iw, ib, sw, sb, pw, pb, ow, ob);
    let mut cursor = Cursor::new(Vec::<u8>::new());
    match write_network(&mut cursor, &network) {
        Ok(()) => {
            cursor.set_position(0);
            match read_network(&mut cursor) {
                Ok(network2) => {
                    assert_eq!(network.iw().row_count(), network2.iw().row_count());
                    assert_eq!(network.iw().col_count(), network2.iw().col_count());
                    assert_eq!(network.iw().elems(), network2.iw().elems());
                    assert_eq!(network.ib().row_count(), network2.ib().row_count());
                    assert_eq!(network.ib().col_count(), network2.ib().col_count());
                    assert_eq!(network.ib().elems(), network2.ib().elems());
                    assert_eq!(network.sw().row_count(), network2.sw().row_count());
                    assert_eq!(network.sw().col_count(), network2.sw().col_count());
                    assert_eq!(network.sw().elems(), network2.sw().elems());
                    assert_eq!(network.sb().row_count(), network2.sb().row_count());
                    assert_eq!(network.sb().col_count(), network2.sb().col_count());
                    assert_eq!(network.sb().elems(), network2.sb().elems());
                    assert_eq!(network.pw().row_count(), network2.pw().row_count());
                    assert_eq!(network.pw().col_count(), network2.pw().col_count());
                    assert_eq!(network.pw().elems(), network2.pw().elems());
                    assert_eq!(network.pb().row_count(), network2.pb().row_count());
                    assert_eq!(network.pb().col_count(), network2.pb().col_count());
                    assert_eq!(network.pb().elems(), network2.pb().elems());
                    assert_eq!(network.ow().row_count(), network2.ow().row_count());
                    assert_eq!(network.ow().col_count(), network2.ow().col_count());
                    assert_eq!(network.ow().elems(), network2.ow().elems());
                    assert_eq!(network.ob().row_count(), network2.ob().row_count());
                    assert_eq!(network.ob().col_count(), network2.ob().col_count());
                    assert_eq!(network.ob().elems(), network2.ob().elems());
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_write_network_v2_and_read_network_v2_writes_network_v2_and_reads_network_v2()
{
    let mut iw_elems = vec![0.0f32; 200 * 100];
    xavier_init(iw_elems.as_mut_slice(), 100, 200);
    let iw = Matrix::new_with_elems(200, 100, iw_elems.as_slice());
    let mut ib_elems = vec![0.0f32; 200];
    xavier_init(ib_elems.as_mut_slice(), 100, 200);
    let ib = Matrix::new_with_elems(200, 1, ib_elems.as_slice());
    let mut ow_elems = vec![0.0f32; 150 * 200];
    xavier_sqrt_init(ow_elems.as_mut_slice(), 200, 150);
    let ow = Matrix::new_with_elems(150, 200, ow_elems.as_slice());
    let mut ob_elems = vec![0.0f32; 150];
    xavier_sqrt_init(ob_elems.as_mut_slice(), 200, 150);
    let ob = Matrix::new_with_elems(150, 1, ob_elems.as_slice());
    let network = NetworkV2::new(iw, ib, ow, ob);
    let mut cursor = Cursor::new(Vec::<u8>::new());
    match write_network_v2(&mut cursor, &network) {
        Ok(()) => {
            cursor.set_position(0);
            match read_network_v2(&mut cursor) {
                Ok(network2) => {
                    assert_eq!(network.iw().row_count(), network2.iw().row_count());
                    assert_eq!(network.iw().col_count(), network2.iw().col_count());
                    assert_eq!(network.iw().elems(), network2.iw().elems());
                    assert_eq!(network.ib().row_count(), network2.ib().row_count());
                    assert_eq!(network.ib().col_count(), network2.ib().col_count());
                    assert_eq!(network.ib().elems(), network2.ib().elems());
                    assert_eq!(network.ow().row_count(), network2.ow().row_count());
                    assert_eq!(network.ow().col_count(), network2.ow().col_count());
                    assert_eq!(network.ow().elems(), network2.ow().elems());
                    assert_eq!(network.ob().row_count(), network2.ob().row_count());
                    assert_eq!(network.ob().col_count(), network2.ob().col_count());
                    assert_eq!(network.ob().elems(), network2.ob().elems());
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}
