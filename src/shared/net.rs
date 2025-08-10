//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::matrix::Matrix;
use crate::shared::Interruption;

/// A trait of neural network.
///
/// This trait provides methods which operate on the neural network.
pub trait Net
{
    /// Computes matrices of hidden layers and output matrices for the neural network.
    ///
    /// The matrices of hidden layers and the output matrices are passed to closures.
    fn compute<HF, OF>(&self, i: &Matrix, depth: usize, pv_count: usize, hf: HF, of: OF) -> Result<(), Interruption>
        where HF: FnMut(Matrix) -> Result<(), Interruption>,
            OF: FnMut(Matrix) -> Result<(), Interruption>;
    
    /// Computes a backpropagation for the neural network.
    ///
    /// The backpropagation uses the input matrix, the matrices of hidden layers, the output
    /// matrices, and the expected output matrices. A `one` matrix should have number of rows equal
    /// to number of columns of input matrix and one column. Also, this matrix should be filled
    /// ones.
    fn backpropagate(&self, i: &Matrix, hs: &[Matrix], os: &[Matrix], ys: &[Matrix], one: &Matrix) -> Self;

    /// Computes an operator for the nerual network.
    fn op<F>(&self, network: &Self, f: F) -> Self
        where F: FnMut(&Matrix, &Matrix) -> Matrix;

    /// Computes an operator with assigment for the neural network.
    fn op_assign<F>(&mut self, network: &Self, f: F)
        where F: FnMut(&mut Matrix, &Matrix);

    /// Computes a function for the neural network.
    fn fun<F>(&self, f: F) -> Self
        where F: FnMut(&Matrix) -> Matrix;

    /// Checks the neural network.
    fn check(&self, input_count: usize, output_count: usize) -> bool;
}
