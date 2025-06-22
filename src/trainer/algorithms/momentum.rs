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
use crate::trainer::net_create::*;
use crate::trainer::TrainerResult;

#[derive(Copy, Clone, Debug)]
pub struct MomentumAlgFactory<T, U, GAF, NL, NF>
{
    params_loader: MomentumParamsLoader,
    state_loader: MomentumStateLoader,
    gradient_adder_factory: GAF,
    net_loader: NL,
    zero_net_factory: NF,
    _unused1: PhantomData<T>,
    _unused2: PhantomData<U>,
}

impl<T, U, GAF, NL, NF> MomentumAlgFactory<T, U, GAF, NL, NF>
{
    pub fn new(gradient_adder_factory: GAF, net_loader: NL, zero_net_factory: NF) -> Self
    {
        MomentumAlgFactory {
            params_loader: MomentumParamsLoader,
            state_loader: MomentumStateLoader,
            gradient_adder_factory,
            net_loader,
            zero_net_factory,
            _unused1: PhantomData::<T>,
            _unused2: PhantomData::<U>,
        }
    }
}

impl<T, U, GAF: GradientAddCreate<U>, NL: Load<T>, NF: NetCreate<T>> MomentumAlgFactory<T, U, GAF, NL, NF>
{
    pub fn create(&self, intr_checker: Arc<dyn IntrCheck + Send + Sync>, converter: Converter) -> Result<MomentumAlg<T, U>>
    {
        let v = load_or_else(&self.net_loader, "v.nnet", || self.zero_net_factory.create(Converter::BOARD_ROW_COUNT, converter.move_row_count()))?; 
        let gradient_adder = self.gradient_adder_factory.create(intr_checker, converter)?;
        let params = self.params_loader.load(PARAMS_NAME)?;
        let state = load_or(&self.state_loader, STATE_NAME, MomentumState { epoch: 1, })?;
        Ok(MomentumAlg::new(gradient_adder, params, state, v))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MomentumParamsLoader;

impl MomentumParamsLoader
{
    pub fn new() -> Self
    { MomentumParamsLoader }
}

impl Load<MomentumParams> for MomentumParamsLoader
{
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<MomentumParams>
    { load_params(path) }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MomentumParams
{
    pub eta: f32,
    pub beta: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct MomentumStateLoader;

impl MomentumStateLoader
{
    pub fn new() -> Self
    { MomentumStateLoader }
}

impl Load<MomentumState> for MomentumStateLoader
{
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<MomentumState>
    { load_state(path) }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct MomentumState
{
    pub epoch: usize,
}

impl Save for MomentumState
{
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    { save_state(path, &self) }
}

pub struct MomentumAlg<T, U>
{
    gradient_adder: U,
    params: MomentumParams,
    state: Mutex<MomentumState>,
    v: Mutex<T>,
    _unused: PhantomData<T>, 
}

impl<T, U> MomentumAlg<T, U>
{
    pub fn new(gradient_adder: U, params: MomentumParams, state: MomentumState, v: T) -> Self
    {
        MomentumAlg {
            gradient_adder,
            params,
            state: Mutex::new(state),
            v: Mutex::new(v), 
            _unused: PhantomData::<T>,
        }
    }
}

impl<T: Net + Save + Send + Sync, U: GradientAdd + GradientPair<T> + Send + Sync> Algorithm for MomentumAlg<T, U>
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
        })?;
        {
            let v_g = self.v.lock().unwrap();
            move_prev_and_save("v", ".nnet", &*v_g)?;
        }
        Ok(())
    }

    fn do_algorithm(&self) -> TrainerResult<()>
    {
        self.gradient_adder.network_and_gradient_in(|network, gradient| {
                let mut v_g = self.v.lock().unwrap();
                v_g.op_assign(gradient, |v, g| *v = &*v * self.params.beta + g);
                network.op_assign(&*v_g, |x, v| *x -= v * self.params.eta);
                let mut state_g = self.state.lock().unwrap();
                state_g.epoch += 1;
        })
    }
}
