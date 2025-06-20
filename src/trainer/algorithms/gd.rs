//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::Result;
use std::marker::PhantomData;
use std::path::Path;
use std::sync::Mutex;
use std::sync::Arc;
use crate::serde::Deserialize;
use crate::serde::Serialize;
use crate::shared::converter::*;
use crate::shared::intr_check::*;
use crate::shared::io::*;
use crate::shared::net::*;
use crate::trainer::algorithm::*;
use crate::trainer::gradient_add::*;
use crate::trainer::gradient_add_create::*;
use crate::trainer::gradient_pair::*;
use crate::trainer::io::*;
use crate::trainer::TrainerResult;

#[derive(Copy, Clone, Debug)]
pub struct GdAlgFactory<T, U, GAF>
{
    params_loader: GdParamsLoader,
    state_loader: GdStateLoader,
    gradient_adder_factory: GAF,
    _unused1: PhantomData<T>,
    _unused2: PhantomData<U>,
}

impl<T, U, GAF> GdAlgFactory<T, U, GAF>
{
    pub fn new(gradient_adder_factory: GAF) -> Self
    {
        GdAlgFactory {
            params_loader: GdParamsLoader,
            state_loader: GdStateLoader,
            gradient_adder_factory,
            _unused1: PhantomData::<T>,
            _unused2: PhantomData::<U>,
        }
    }
}

impl<T, U, GAF: GradientAddCreate<U>> GdAlgFactory<T, U, GAF>
{
    pub fn create(&self, intr_checker: Arc<dyn IntrCheck + Send + Sync>, converter: Converter) -> Result<GdAlg<T, U>>
    {
        let gradient_adder = self.gradient_adder_factory.create(intr_checker, converter)?;
        let params = self.params_loader.load(PARAMS_NAME)?;
        let state = load_or(&self.state_loader, STATE_NAME, GdState { epoch: 1, })?;
        Ok(GdAlg::new(gradient_adder, params, state))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GdParamsLoader;

impl GdParamsLoader
{
    pub fn new() -> Self
    { GdParamsLoader }
}

impl Load<GdParams> for GdParamsLoader
{
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<GdParams>
    { load_params(path) }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GdParams
{
    pub eta: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct GdStateLoader;

impl GdStateLoader
{
    pub fn new() -> Self
    { GdStateLoader }
}

impl Load<GdState> for GdStateLoader
{
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<GdState>
    { load_state(path) }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct GdState
{
    pub epoch: usize,
}

impl Save for GdState
{
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    { save_state(path, &self) }
}

pub struct GdAlg<T, U>
{
    gradient_adder: U,
    params: GdParams,
    state: Mutex<GdState>,
    _unused: PhantomData<T>, 
}

impl<T, U> GdAlg<T, U>
{
    pub fn new(gradient_adder: U, params: GdParams, state: GdState) -> Self
    {
        GdAlg {
            gradient_adder,
            params,
            state: Mutex::new(state),
            _unused: PhantomData::<T>,
        }
    }
}

impl<T: Net + Save + Send + Sync, U: GradientAdd + GradientPair<T> + Send + Sync> Algorithm for GdAlg<T, U>
{
    fn gradient_adder(&self) -> &(dyn GradientAdd + Send + Sync)
    { &self.gradient_adder }

    fn epoch(&self) -> usize
    {
        let state_g = self.state.lock().unwrap();
        state_g.epoch
    }
    
    fn save(&self) -> Result<()>
    {
        {
            let state_g = self.state.lock().unwrap();
            move_prev_and_save(STATE_NAME_PREFIX, STATE_NAME_SUFFIX, &*state_g)?;
        }
        self.gradient_adder.network_in(|network| {
                move_prev_and_save(NETWORK_NAME_PREFIX, NETWORK_NAME_SUFFIX, network)
        })
    }

    fn do_algorithm(&self) -> TrainerResult<()>
    {
        self.gradient_adder.network_and_gradient_in(|network, gradient| {
                network.op_assign(gradient, |x, g| *x -= g * self.params.eta);
                let mut state_g = self.state.lock().unwrap();
                state_g.epoch += 1;
        })
    }
}
