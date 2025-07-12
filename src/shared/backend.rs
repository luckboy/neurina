//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::sync::Arc;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::CL_DEVICE_TYPE_ALL;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::ClBackend;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::Context;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::Device;
#[cfg(feature = "opencl")]
use crate::matrix::opencl::get_platforms;
#[cfg(feature = "cuda")]
use crate::matrix::cuda::CudaBackend;
#[cfg(feature = "opencl")]
use crate::matrix::Error;
use crate::matrix::Result;
use crate::matrix::set_default_backend;
use crate::matrix::unset_default_backend;
use crate::shared::config::*;

#[cfg(feature = "cuda")]
fn initialize_cuda_backend(ordinal: usize, is_cublas: bool, is_mma: bool) -> Result<()>
{ set_default_backend(Arc::new(CudaBackend::new_with_ordinal_and_flags(ordinal, is_cublas, is_mma)?)) }

#[cfg(feature = "opencl")]
fn initialize_opencl_backend(platform_idx: usize, device_idx: usize) -> Result<()>
{
    let platforms = match get_platforms() {
        Ok(tmp_platforms) => tmp_platforms,
        Err(err) => return Err(Error::OpenCl(err)),
    };
    let platform = match platforms.get(platform_idx) {
        Some(tmp_platform) => tmp_platform,
        None => return Err(Error::NoPlatform),
    };
    let device_ids = match platform.get_devices(CL_DEVICE_TYPE_ALL) {
        Ok(tmp_device_ids) => tmp_device_ids,
        Err(err) => return Err(Error::OpenCl(err)),
    };
    let device = match device_ids.get(device_idx) {
        Some(device_id) => Device::new(*device_id),
        None => return Err(Error::NoDevice),
    };
    let context = match Context::from_device(&device) {
        Ok(tmp_context) => tmp_context,
        Err(err) => return Err(Error::OpenCl(err)),
    };
    set_default_backend(Arc::new(ClBackend::new_with_context(context)?))
}

/// Initializes a backend for operations on matrices.
#[allow(unused_assignments)]
pub fn initialize_backend(config: &Option<Config>) -> Result<()>
{
    let mut is_first_opencl = false;
    let mut ordinal = 0usize;
    let mut platform_idx = 0usize;
    let mut device_idx = 0usize;
    let mut is_cublas = true;
    let mut is_mma = false;
    match config {
        Some(config) => {
            match &config.backend {
                Some(backend_config) => {
                    is_first_opencl = backend_config.first_opencl.unwrap_or(is_first_opencl);
                    ordinal = backend_config.ordinal.unwrap_or(ordinal);
                    platform_idx = backend_config.platform.unwrap_or(platform_idx);
                    device_idx = backend_config.device.unwrap_or(device_idx);
                    is_cublas = backend_config.cublas.unwrap_or(is_cublas);
                    is_mma = backend_config.mma.unwrap_or(is_mma);
                },
                None => (),
            }
        },
        None => (),
    }
    let mut res: Option<Result<()>> = None;
    #[cfg(feature = "opencl")]
    if is_first_opencl {
        match res {
            Some(Err(_)) | None => res = Some(initialize_opencl_backend(platform_idx, device_idx)),
            _ => (),
        }
    }
    #[cfg(feature = "cuda")]
    match res {
        Some(Err(_)) | None => res = Some(initialize_cuda_backend(ordinal, is_cublas, is_mma)),
        _ => (),
    }
    #[cfg(feature = "opencl")]
    if !is_first_opencl {
        match res {
            Some(Err(_)) | None => res = Some(initialize_opencl_backend(platform_idx, device_idx)),
            _ => (),
        }
    }
    res.unwrap()
}

/// Finalizes a backend for operations on matrices.
pub fn finalize_backend() -> Result<()>
{ unset_default_backend() }
