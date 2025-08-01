//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use crate::matrix::Matrix;
use crate::shared::Network;

/// A loader trait.
///
/// This trait provides method that loads the specified data from the file.
pub trait Load<T>
{
    /// Loads the specified data from the file.
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<T>;
}

/// A saver trait.
///
/// This trait provides method that saves the specified data to the file.
pub trait Save
{
    /// Saves the specified data to the file.
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}

/// A structure of loader of neural network.
///
/// The loader of neural network loads a neural network.
#[derive(Copy, Clone, Debug)]
pub struct NetworkLoader;

impl NetworkLoader
{
    /// Creates a loader of neural network.
    pub fn new() -> Self
    { NetworkLoader }
}

impl Load<Network> for NetworkLoader
{
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<Network>
    { load_network(path) }
}

/// Reads a matrix from the reader.
pub fn read_matrix(r: &mut dyn Read) -> Result<Matrix>
{
    let mut u64_buf: [u8; 8] = [0; 8];
    r.read_exact(&mut u64_buf)?;
    let row_count_u64 = u64::from_le_bytes(u64_buf);
    r.read_exact(&mut u64_buf)?;
    let col_count_u64 = u64::from_le_bytes(u64_buf);
    if row_count_u64 >= usize::MAX as u64 {
        return Err(Error::new(ErrorKind::InvalidData, "too many rows"));
    }
    if col_count_u64 >= usize::MAX as u64 {
        return Err(Error::new(ErrorKind::InvalidData, "too many columns"));
    }
    let row_count = row_count_u64 as usize;
    let col_count = col_count_u64 as usize;
    let elem_count = match row_count.checked_mul(col_count) {
        Some(tmp_elem_count) => tmp_elem_count,
        None => return Err(Error::new(ErrorKind::InvalidData, "too many elements")),
    };
    let mut elems = vec![0.0f32; elem_count];
    for i in 0..elems.len() {
        let mut f32_buf: [u8; 4] = [0; 4];
        r.read_exact(&mut f32_buf)?;
        elems[i] = f32::from_le_bytes(f32_buf);
    }
    Ok(Matrix::new_with_elems(row_count, col_count, elems.as_slice()))
}

/// Writes the matrix to the writer.
pub fn write_matrix(w: &mut dyn Write, matrix: &Matrix) -> Result<()>
{
    let mut u64_buf = (matrix.row_count() as u64).to_le_bytes();
    w.write_all(&u64_buf)?;
    u64_buf = (matrix.col_count() as u64).to_le_bytes();
    w.write_all(&u64_buf)?;
    let elems = matrix.elems();
    for i in 0..elems.len() {
        let f32_buf = elems[i].to_le_bytes();
        w.write_all(&f32_buf)?;
    }
    Ok(())
}

/// Reads a neural network from the reader.
pub fn read_network(r: &mut dyn Read) -> Result<Network>
{
    let mut magic_buf: [u8; 12] = [0; 12];
    r.read_exact(&mut magic_buf)?;
    if &magic_buf != b"neurina_v001" {
        return Err(Error::new(ErrorKind::InvalidData, "invalid network format"));
    }
    let iw = read_matrix(r)?;
    let ib = read_matrix(r)?;
    let sw = read_matrix(r)?;
    let sb = read_matrix(r)?;
    let pw = read_matrix(r)?;
    let pb = read_matrix(r)?;
    let ow = read_matrix(r)?;
    let ob = read_matrix(r)?;
    Ok(Network::new(iw, ib, sw, sb, pw, pb, ow, ob))
}

/// Writes the neural network to the writer.
pub fn write_network(w: &mut dyn Write, network: &Network) -> Result<()>
{
    w.write_all(b"neurina_v001")?;
    write_matrix(w, network.iw())?;
    write_matrix(w, network.ib())?;
    write_matrix(w, network.sw())?;
    write_matrix(w, network.sb())?;
    write_matrix(w, network.pw())?;
    write_matrix(w, network.pb())?;
    write_matrix(w, network.ow())?;
    write_matrix(w, network.ob())?;
    Ok(())
}

/// Loads a neural network from the file.
pub fn load_network<P: AsRef<Path>>(path: P) -> Result<Network>
{
    let file = File::open(path)?;
    let mut r = BufReader::new(file);
    read_network(&mut r)
}

/// Saves the neural network to the file.
pub fn save_network<P: AsRef<Path>>(path: P, network: &Network) -> Result<()>
{
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);
    write_network(&mut w, network)
}

#[cfg(test)]
mod tests;
