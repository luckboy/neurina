//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Cursor;
use super::*;

#[test]
fn test_read_config_reads_configuration_file()
{
    let s = "
[backend]
platform = 1
device = 2
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2);
    match read_config(&mut cursor) {
        Ok(config) => {
            match &config.backend {
                Some(backend_config) => {
                    assert_eq!(None, backend_config.first_opencl);
                    assert_eq!(None, backend_config.ordinal);
                    assert_eq!(Some(1), backend_config.platform);
                    assert_eq!(Some(2), backend_config.device);
                    assert_eq!(None, backend_config.cublas);
                    assert_eq!(None, backend_config.mma);
                },
                None => assert!(false),
            } 
        },
        Err(err) => assert!(false),
    }
}

#[test]
fn test_read_config_reads_configuration_file_for_all_backend_fields()
{
    let s = "
[backend]
first_opencl = true
ordinal = 1
platform = 2
device = 3
cublas = false
mma = true
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2);
    match read_config(&mut cursor) {
        Ok(config) => {
            match &config.backend {
                Some(backend_config) => {
                    assert_eq!(Some(true), backend_config.first_opencl);
                    assert_eq!(Some(1), backend_config.ordinal);
                    assert_eq!(Some(2), backend_config.platform);
                    assert_eq!(Some(3), backend_config.device);
                    assert_eq!(Some(false), backend_config.cublas);
                    assert_eq!(Some(true), backend_config.mma);
                },
                None => assert!(false),
            } 
        },
        Err(err) => assert!(false),
    }
}
