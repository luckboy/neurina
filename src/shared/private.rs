//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Error;
use std::io::ErrorKind;

pub (crate) fn csv_error_to_io_error(err: csv::Error) -> Error
{
    if err.is_io_error() {
        match err.into_kind() {
            csv::ErrorKind::Io(err) => err,
            _ => Error::new(ErrorKind::InvalidData, String::from("csv error: unknown error")),
        }
    } else {
        Error::new(ErrorKind::InvalidData, format!("csv error: {}", err))
    }
}
