//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_matrix_buffer_do_elems_does_elements()
{
    let mut elems = vec![2.0f32, 1.0f32, 4.0f32, 3.0f32, 6.0f32, 5.0f32];
    let mut inputs: Vec<Matrix> = Vec::new();
    let mut matrix_buf = MatrixBuffer::new(3, 0, 4, 0, ());
    let intr_checker = EmptyIntrChecker::new();
    matrix_buf.do_elems(elems.as_mut_slice(), 0, &intr_checker, |e, i, _, j, col_count| {
            i[0 * col_count + j] = *e;
            i[1 * col_count + j] = *e;
            i[2 * col_count + j] = *e;
    }, |i, _, _, es| {
            inputs.push(i);
            for e in es {
                *e += 1.0;
            }
            Ok(())
    }).unwrap();
    assert_eq!(vec![3.0f32, 2.0f32, 5.0f32, 4.0f32, 7.0f32, 6.0f32], elems);
    assert_eq!(2, inputs.len());
    assert_eq!(vec![2.0f32, 1.0f32, 4.0f32, 3.0f32, 2.0f32, 1.0f32, 4.0f32, 3.0f32, 2.0f32, 1.0f32, 4.0f32, 3.0f32], inputs[0].elems());
    assert_eq!(vec![6.0f32, 5.0f32, 6.0f32, 5.0f32, 6.0f32, 5.0f32], inputs[1].elems());
}

#[test]
fn test_matrix_buffer_do_elems_does_elements_with_outputs()
{
    let mut elems = vec![2.0f32, 1.0f32, 4.0f32, 3.0f32, 6.0f32, 5.0f32];
    let mut inputs: Vec<Matrix> = Vec::new();
    let mut outputs: Vec<Vec<Matrix>> = Vec::new();
    let mut matrix_buf = MatrixBuffer::new(3, 2, 4, 2, ());
    let intr_checker = EmptyIntrChecker::new();
    matrix_buf.do_elems(elems.as_mut_slice(), 2, &intr_checker, |e, i, os, j, col_count| {
            i[0 * col_count + j] = *e;
            i[1 * col_count + j] = *e;
            i[2 * col_count + j] = *e;
            os[0][0 * col_count + j] = *e + 1.0;
            os[0][1 * col_count + j] = *e + 1.0;
            os[1][0 * col_count + j] = *e + 2.0;
            os[1][1 * col_count + j] = *e + 2.0;
    }, |i, os, _, es| {
            inputs.push(i);
            outputs.push(os.to_vec());
            for e in es {
                *e += 1.0;
            }
            Ok(())
    }).unwrap();
    assert_eq!(vec![3.0f32, 2.0f32, 5.0f32, 4.0f32, 7.0f32, 6.0f32], elems);
    assert_eq!(2, inputs.len());
    assert_eq!(vec![2.0f32, 1.0f32, 4.0f32, 3.0f32, 2.0f32, 1.0f32, 4.0f32, 3.0f32, 2.0f32, 1.0f32, 4.0f32, 3.0f32], inputs[0].elems());
    assert_eq!(vec![6.0f32, 5.0f32, 6.0f32, 5.0f32, 6.0f32, 5.0f32], inputs[1].elems());
    assert_eq!(2, outputs.len());
    assert_eq!(2, outputs[0].len());
    assert_eq!(vec![3.0f32, 2.0f32, 5.0f32, 4.0f32, 3.0f32, 2.0f32, 5.0f32, 4.0f32], outputs[0][0].elems());
    assert_eq!(vec![4.0f32, 3.0f32, 6.0f32, 5.0f32, 4.0f32, 3.0f32, 6.0f32, 5.0f32], outputs[0][1].elems());
    assert_eq!(2, outputs[1].len());
    assert_eq!(vec![7.0f32, 6.0f32, 7.0f32, 6.0f32], outputs[1][0].elems());
    assert_eq!(vec![8.0f32, 7.0f32, 8.0f32, 7.0f32], outputs[1][1].elems());
}

#[test]
fn test_matrix_buffer_do_elems_does_elements_with_outputs_after_resize_output_bufs()
{
    let mut elems = vec![2.0f32, 1.0f32, 4.0f32, 3.0f32, 6.0f32, 5.0f32];
    let mut inputs: Vec<Matrix> = Vec::new();
    let mut outputs: Vec<Vec<Matrix>> = Vec::new();
    let mut matrix_buf = MatrixBuffer::new(3, 2, 4, 1, ());
    matrix_buf.resize_output_bufs(2); 
    let intr_checker = EmptyIntrChecker::new();
    matrix_buf.do_elems(elems.as_mut_slice(), 2, &intr_checker, |e, i, os, j, col_count| {
            i[0 * col_count + j] = *e;
            i[1 * col_count + j] = *e;
            i[2 * col_count + j] = *e;
            os[0][0 * col_count + j] = *e + 1.0;
            os[0][1 * col_count + j] = *e + 1.0;
            os[1][0 * col_count + j] = *e + 2.0;
            os[1][1 * col_count + j] = *e + 2.0;
    }, |i, os, _, es| {
            inputs.push(i);
            outputs.push(os.to_vec());
            for e in es {
                *e += 1.0;
            }
            Ok(())
    }).unwrap();
    assert_eq!(vec![3.0f32, 2.0f32, 5.0f32, 4.0f32, 7.0f32, 6.0f32], elems);
    assert_eq!(2, inputs.len());
    assert_eq!(vec![2.0f32, 1.0f32, 4.0f32, 3.0f32, 2.0f32, 1.0f32, 4.0f32, 3.0f32, 2.0f32, 1.0f32, 4.0f32, 3.0f32], inputs[0].elems());
    assert_eq!(vec![6.0f32, 5.0f32, 6.0f32, 5.0f32, 6.0f32, 5.0f32], inputs[1].elems());
    assert_eq!(2, outputs.len());
    assert_eq!(2, outputs[0].len());
    assert_eq!(vec![3.0f32, 2.0f32, 5.0f32, 4.0f32, 3.0f32, 2.0f32, 5.0f32, 4.0f32], outputs[0][0].elems());
    assert_eq!(vec![4.0f32, 3.0f32, 6.0f32, 5.0f32, 4.0f32, 3.0f32, 6.0f32, 5.0f32], outputs[0][1].elems());
    assert_eq!(2, outputs[1].len());
    assert_eq!(vec![7.0f32, 6.0f32, 7.0f32, 6.0f32], outputs[1][0].elems());
    assert_eq!(vec![8.0f32, 7.0f32, 8.0f32, 7.0f32], outputs[1][1].elems());
}
