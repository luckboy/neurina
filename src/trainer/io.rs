//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::metadata;
use std::fs::remove_file;
use std::fs::rename;
use std::io::ErrorKind;
use std::io::Result;
use crate::shared::io::*;

pub fn load_or<T, L: Load<T>>(loader: L, file_name: &str, value: T) -> Result<T>
{
    match loader.load(file_name) {
        Ok(tmp_value) => Ok(tmp_value),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(value),
        Err(err) => Err(err),
    }
}

pub fn move_prev_and_save<T: Save>(prefix: &str, suffix: &str, value: &T) -> Result<()>
{
    let prev_file_name = format!("{}-2{}", prefix, suffix);
    let file_name = format!("{}{}", prefix, suffix);
    match metadata(file_name.as_str()) {
        Ok(_) => {
            remove_file(prev_file_name.as_str())?;
            rename(file_name.as_str(), prev_file_name.as_str())?;
        },
        Err(err) if err.kind() == ErrorKind::NotFound => (),
        Err(err) => return Err(err),
    }
    value.save(file_name.as_str())?;
    Ok(())
}
