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

#[derive(Clone, Debug, Deserialize)]
pub struct Config
{
    pub backend: Option<BackendConfig>,
    pub syzygy: Option<SyzygyConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BackendConfig
{
    pub first_opencl: Option<bool>,
    pub ordinal: Option<usize>,
    pub platform: Option<usize>,
    pub device: Option<usize>,
    pub cublas: Option<bool>,
    pub mma: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SyzygyConfig
{
    pub path: Option<String>
}

pub fn read_config(r: &mut dyn Read) -> Result<Config>
{
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    match toml::from_str::<Config>(s.as_str()) {
        Ok(config) => Ok(config),
        Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("toml error: {}", err))),
    }
}

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
