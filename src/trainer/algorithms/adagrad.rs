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
pub struct AdagradAlgFactory<T, U, GAF, NL, NF>
{
    params_loader: AdagradParamsLoader,
    state_loader: AdagradStateLoader,
    gradient_adder_factory: GAF,
    net_loader: NL,
    zero_net_factory: NF,
    _unused1: PhantomData<T>,
    _unused2: PhantomData<U>,
}

impl<T, U, GAF, NL, NF> AdagradAlgFactory<T, U, GAF, NL, NF>
{
    pub fn new(gradient_adder_factory: GAF, net_loader: NL, zero_net_factory: NF) -> Self
    {
        AdagradAlgFactory {
            params_loader: AdagradParamsLoader,
            state_loader: AdagradStateLoader,
            gradient_adder_factory,
            net_loader,
            zero_net_factory,
            _unused1: PhantomData::<T>,
            _unused2: PhantomData::<U>,
        }
    }
}

impl<T, U, GAF: GradientAddCreate<U>, NL: Load<T>, NF: NetCreate<T>> AdagradAlgFactory<T, U, GAF, NL, NF>
{
    pub fn create(&self, intr_checker: Arc<dyn IntrCheck + Send + Sync>, converter: Converter) -> Result<AdagradAlg<T, U>>
    {
        let s = load_or_else(&self.net_loader, "s.nnet", || self.zero_net_factory.create(Converter::BOARD_ROW_COUNT, converter.move_row_count()))?; 
        let gradient_adder = self.gradient_adder_factory.create(intr_checker, converter)?;
        let params = self.params_loader.load(PARAMS_NAME)?;
        let state = load_or(&self.state_loader, STATE_NAME, AdagradState { epoch: 1, })?;
        Ok(AdagradAlg::new(gradient_adder, params, state, s))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AdagradParamsLoader;

impl AdagradParamsLoader
{
    pub fn new() -> Self
    { AdagradParamsLoader }
}

impl Load<AdagradParams> for AdagradParamsLoader
{
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<AdagradParams>
    { load_params(path) }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct AdagradParams
{
    pub eta: f32,
    pub eps: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct AdagradStateLoader;

impl AdagradStateLoader
{
    pub fn new() -> Self
    { AdagradStateLoader }
}

impl Load<AdagradState> for AdagradStateLoader
{
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<AdagradState>
    { load_state(path) }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct AdagradState
{
    pub epoch: usize,
}

impl Save for AdagradState
{
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<()>
    { save_state(path, &self) }
}

pub struct AdagradAlg<T, U>
{
    gradient_adder: U,
    params: AdagradParams,
    state: Mutex<AdagradState>,
    s: Mutex<T>,
    _unused: PhantomData<T>, 
}

impl<T, U> AdagradAlg<T, U>
{
    pub fn new(gradient_adder: U, params: AdagradParams, state: AdagradState, s: T) -> Self
    {
        AdagradAlg {
            gradient_adder,
            params,
            state: Mutex::new(state),
            s: Mutex::new(s), 
            _unused: PhantomData::<T>,
        }
    }
}

impl<T: Net + Save + Send + Sync, U: GradientAdd + GradientPair<T> + Send + Sync> Algorithm for AdagradAlg<T, U>
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
            let s_g = self.s.lock().unwrap();
            move_prev_and_save("s", ".nnet", &*s_g)?;
        }
        Ok(())
    }

    fn do_algorithm(&self) -> TrainerResult<()>
    {
        self.gradient_adder.network_and_gradient_in(|network, gradient| {
                let mut s_g = self.s.lock().unwrap();
                s_g.op_assign(gradient, |s, g| *s += g.mul_elems(g));
                let tmp = s_g.op(gradient, |s, g| (s + self.params.eps).sqrt().rdiv(self.params.eta).mul_elems(g));
                network.op_assign(&tmp, |x, t| *x -= t);
                let mut state_g = self.state.lock().unwrap();
                state_g.epoch += 1;
        })
    }
}
