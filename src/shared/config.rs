//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::path::Path;
use crate::serde::Deserialize;

/// A structure of configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct Config
{
    /// A backend configuration.
    pub backend: Option<BackendConfig>,
    /// A configuration of Syzygy endgame tablebases.
    pub syzygy: Option<SyzygyConfig>,
}

/// A structure of backend configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct BackendConfig
{
    /// If this field is `true`, there tries to initialize the OpenCL backend as first. Default
    /// value of this field is `true`.
    pub first_opencl: Option<bool>,
    /// A ordinal number for the CUDA backend. Default value of this field is zero.
    pub ordinal: Option<usize>,
    /// A platform index for the OpenCL backend. Default value of this field is zero.
    pub platform: Option<usize>,
    /// A decive index for the OpenCL backend. Default value of this field is zero.
    pub device: Option<usize>,
    /// If this field is `true`, the CUDA backend uses the cuBLAS library. Default value of this
    /// field is `true`.
    pub cublas: Option<bool>,
    /// If this field is `true`, the CUDA backend uses the mma instruction. Default values of this
    /// field is `false`.
    pub mma: Option<bool>,
}

/// A structure of configuration of Syzygy endgame tablebases.
#[derive(Clone, Debug, Deserialize)]
pub struct SyzygyConfig
{
    /// A path to the Syzygy endgame tablebases.
    pub path: Option<String>,
}

/// Reads a configuration from the reader.
pub fn read_config(r: &mut dyn Read) -> Result<Config>
{
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    match toml::from_str::<Config>(s.as_str()) {
        Ok(config) => Ok(config),
        Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("toml error: {}", err))),
    }
}

/// Loads a configuration from the file.
///
/// If the configuration file doesn't exist, this method returns `None`.
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Option<Config>>
{
    match File::open(path) {
        Ok(mut file) => Ok(Some(read_config(&mut file)?)),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests;
