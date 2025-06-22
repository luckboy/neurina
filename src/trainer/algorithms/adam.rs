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
pub struct AdamAlgFactory<T, U, GAF, NL, NF>
{
    params_loader: AdamParamsLoader,
    state_loader: AdamStateLoader,
    gradient_adder_factory: GAF,
    net_loader: NL,
    zero_net_factory: NF,
    _unused1: PhantomData<T>,
    _unused2: PhantomData<U>,
}

impl<T, U, GAF, NL, NF> AdamAlgFactory<T, U, GAF, NL, NF>
{
    pub fn new(gradient_adder_factory: GAF, net_loader: NL, zero_net_factory: NF) -> Self
    {
        AdamAlgFactory {
            params_loader: AdamParamsLoader,
            state_loader: AdamStateLoader,
            gradient_adder_factory,
            net_loader,
            zero_net_factory,
            _unused1: PhantomData::<T>,
            _unused2: PhantomData::<U>,
        }
    }
}

impl<T, U, GAF: GradientAddCreate<U>, NL: Load<T>, NF: NetCreate<T>> AdamAlgFactory<T, U, GAF, NL, NF>
{
    pub fn create(&self, intr_checker: Arc<dyn IntrCheck + Send + Sync>, converter: Converter) -> Result<AdamAlg<T, U>>
    {
        let v = load_or_else(&self.net_loader, "v.nnet", || self.zero_net_factory.create(Converter::BOARD_ROW_COUNT, converter.move_row_count()))?; 
        let s = load_or_else(&self.net_loader, "s.nnet", || self.zero_net_factory.create(Converter::BOARD_ROW_COUNT, converter.move_row_count()))?; 
        let gradient_adder = self.gradient_adder_factory.create(intr_checker, converter)?;
        let params = self.params_loader.load(PARAMS_NAME)?;
        let state = load_or(&self.state_loader, STATE_NAME, AdamState { epoch: 1, })?;
        Ok(AdamAlg::new(gradient_adder, params, state, v, s))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AdamParamsLoader;

impl AdamParamsLoader
{
    pub fn new() -> Self
    { AdamParamsLoader }
}

impl Load<AdamParams> for AdamParamsLoader
{
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<AdamParams>
    { load_params(path) }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct AdamParams
{
    pub eta: f32,
    pub beta1: f32,
    pub beta2: f32,
    pub eps: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct AdamStateLoader;

impl AdamStateLoader
{
    pub fn new() -> Self
    { AdamStateLoader }
}

impl Load<AdamState> for AdamStateLoader
{
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<AdamState>
    { load_state(path) }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct AdamState
{
    pub epoch: usize,
}

impl Save for AdamState
{
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    { save_state(path, &self) }
}

pub struct AdamAlg<T, U>
{
    gradient_adder: U,
    params: AdamParams,
    state: Mutex<AdamState>,
    v: Mutex<T>,
    s: Mutex<T>,
    _unused: PhantomData<T>, 
}

impl<T, U> AdamAlg<T, U>
{
    pub fn new(gradient_adder: U, params: AdamParams, state: AdamState, v: T, s: T) -> Self
    {
        AdamAlg {
            gradient_adder,
            params,
            state: Mutex::new(state),
            v: Mutex::new(v), 
            s: Mutex::new(s), 
            _unused: PhantomData::<T>,
        }
    }
}

impl<T: Net + Save + Send + Sync, U: GradientAdd + GradientPair<T> + Send + Sync> Algorithm for AdamAlg<T, U>
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
        {
            let s_g = self.s.lock().unwrap();
            move_prev_and_save("s", ".nnet", &*s_g)?;
        }
        Ok(())
    }

    fn do_algorithm(&self) -> TrainerResult<()>
    {
        self.gradient_adder.network_and_gradient_in(|network, gradient| {
                let mut v_g = self.v.lock().unwrap();
                let mut s_g = self.s.lock().unwrap();
                let mut state_g = self.state.lock().unwrap();
                v_g.op_assign(gradient, |v, g| *v = &*v * self.params.beta1 + g * (1.0 - self.params.beta1));
                s_g.op_assign(gradient, |s, g| *s = &*s * self.params.beta2 + g.mul_elems(g) * (1.0 - self.params.beta2));
                let v_bias_corr = v_g.fun(|v| v / (1.0 - self.params.beta1).powf(state_g.epoch as f32));
                let s_bias_corr = s_g.fun(|s| s / (1.0 - self.params.beta2).powf(state_g.epoch as f32));
                let grad_prime = v_bias_corr.op(&s_bias_corr, |vbc, sbc| (vbc * self.params.eta).div_elems(&(sbc.sqrt() + self.params.eps)));
                network.op_assign(&grad_prime, |x, gp| *x -= gp);
                state_g.epoch += 1;
        })
    }
}
