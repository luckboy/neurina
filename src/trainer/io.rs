//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::File;
use std::fs::copy;
use std::fs::metadata;
use std::fs::remove_file;
use std::fs::rename;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use crate::serde::de::DeserializeOwned;
use crate::serde::ser::Serialize;
use crate::shared::io::*;

/// Loads the specified data from the file if the file is existent, otherwise returns the
/// returned specified data by the closure.
pub fn load_or_else<T, L: Load<T>, F>(loader: &L, file_name: &str, f: F) -> Result<T>
    where F: FnOnce() -> T
{
    match loader.load(file_name) {
        Ok(value) => Ok(value),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(f()),
        Err(err) => Err(err),
    }
}

/// Loads the specified data from the file if the file is existent, otherwise returns the specified
/// data.
pub fn load_or<T, L: Load<T>>(loader: &L, file_name: &str, value: T) -> Result<T>
{ load_or_else(loader, file_name, || value) }

/// Renames the current file to the previous file and saves the specified data to the current file.
///
/// The previous file is removed if the previous file and the current file are existent. The
/// current is renamed to the previous file if the current file is existent. The speciefied data is
/// saved as the current file. The previous file name consists of the prefix, the `"-2"` string,
/// and the suffix. The current file name only consists of the prefix and the suffix. 
pub fn move_prev_and_save<T: Save>(prefix: &str, suffix: &str, value: &T) -> Result<()>
{
    let prev_file_name = format!("{}-2{}", prefix, suffix);
    let file_name = format!("{}{}", prefix, suffix);
    match metadata(file_name.as_str()) {
        Ok(_) => {
            match remove_file(prev_file_name.as_str()) {
                Ok(()) => (),
                Err(err) if err.kind() == ErrorKind::NotFound => (),
                Err(err) => return Err(err),
            }
            rename(file_name.as_str(), prev_file_name.as_str())?;
        },
        Err(err) if err.kind() == ErrorKind::NotFound => (),
        Err(err) => return Err(err),
    }
    value.save(file_name.as_str())?;
    Ok(())
}

fn read_params_or_state<T: DeserializeOwned>(r: &mut dyn Read) -> Result<T>
{
    let mut s = String::new();
    r.read_to_string(&mut s)?;
    match toml::from_str::<T>(s.as_str()) {
        Ok(params_or_state) => Ok(params_or_state),
        Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("toml error: {}", err))),
    }
}

/// Reads the parameters from the reader.
pub fn read_params<T: DeserializeOwned>(r: &mut dyn Read) -> Result<T>
{ read_params_or_state(r) }

/// Reads the state from the reader.
pub fn read_state<T: DeserializeOwned>(r: &mut dyn Read) -> Result<T>
{ read_params_or_state(r) }

/// Writes the state to the writer.
pub fn write_state<T: Serialize + ?Sized>(w: &mut dyn Write, state: &T) -> Result<()>
{
    match toml::to_string(state) {
        Ok(s) => write!(w, "{}", s),
        Err(err) => Err(Error::new(ErrorKind::InvalidData, format!("toml error: {}", err))),
    }
}

/// Loads the parameters from the file.
pub fn load_params<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T>
{
    let mut file = File::open(path)?;
    read_params(&mut file)
}

/// Loads the state from the file.
pub fn load_state<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T>
{
    let mut file = File::open(path)?;
    read_state(&mut file)
}

/// Saves the state to the file.
pub fn save_state<P: AsRef<Path>, T: Serialize + ?Sized>(path: P, state: &T) -> Result<()>
{
    let mut file = File::create(path)?;
    write_state(&mut file, state)
}

/// Appends the gnuplot data to the file.
pub fn append_gnuplot_data<P: AsRef<Path>>(path: P, x: usize, y: u64) -> Result<()>
{
    let mut file = File::options().create(true).append(true).open(path)?;
    writeln!(&mut file, "{} {}", x, y)
}

/// Copies the gnuplot data from the old file to the new file and appends the gnuplot data to the
/// new file.
///
/// Copies the gnuplot data from the old file to the new file if the old file is existent. The
/// gnuplot data is appended to the new file.
pub fn copy_and_append_gnuplot_data<P: AsRef<Path>, Q: AsRef<Path>>(old_path: P, new_path: Q, x: usize, y: u64) -> Result<()>
{
    match copy(old_path, new_path.as_ref()) {
        Ok(_) => (),
        Err(err) if err.kind() == ErrorKind::NotFound => (),
        Err(err) => return Err(err),
    }
    append_gnuplot_data(new_path, x, y)
}
