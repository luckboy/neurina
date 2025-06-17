//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use crate::trainer::algorithm::*;
use crate::trainer::data_sample::*;
use crate::trainer::print::*;
use crate::trainer::sample::*;
use crate::trainer::TrainerError;
use crate::trainer::TrainerResult;

pub struct Trainer
{
    sampler: Arc<dyn Sample + Send + Sync>,
    algorithm: Arc<dyn Algorithm + Send + Sync>,
    writer: Arc<Mutex<dyn Write + Send + Sync>>,
    printer: Arc<dyn Print + Send + Sync>,
}

impl Trainer
{
    pub const MINIBATCH_COUNT_TO_PRINT: u64 = 32;
    
    pub fn new(sampler: Arc<dyn Sample + Send + Sync>, algorithm: Arc<dyn Algorithm + Send + Sync>, writer: Arc<Mutex<dyn Write + Send + Sync>>, printer: Arc<dyn Print + Send + Sync>) -> Self
    { Trainer { sampler, algorithm, writer, printer, } }
    
    pub fn sampler(&self) -> &Arc<dyn Sample + Send + Sync>
    { &self.sampler }

    pub fn algorithm(&self) -> &Arc<dyn Algorithm + Send + Sync>
    { &self.algorithm }

    pub fn writer(&self) -> &Arc<Mutex<dyn Write + Send + Sync>>
    { &self.writer }

    pub fn printer(&self) -> &Arc<dyn Print + Send + Sync>
    { &self.printer }
    
    pub fn load(&self) -> TrainerResult<()>
    { self.algorithm.load() }

    pub fn save(&self) -> TrainerResult<()>
    { self.algorithm.save() }
    
    fn do_data(&self, data: &mut dyn Iterator<Item = TrainerResult<Option<DataSample>>>, are_gradients: bool) -> TrainerResult<(u64, u64, u64)>
    {
        let mut minibatches: BTreeMap<usize, Vec<DataSample>> = BTreeMap::new();
        let mut sample_count = 0u64;
        let mut computed_minibatch_count = 0u64;
        let mut minibatch_count = 0u64;
        let mut passed_output_count = 064;
        let mut all_output_count = 064;
        let mut err_count = 0u64;
        {
            let mut writer_g = self.writer.lock().unwrap();
            match self.printer.print(&mut *writer_g, sample_count, computed_minibatch_count, minibatch_count, false) {
                Ok(()) => (),
                Err(err) => return Err(TrainerError::Io(err)),
            }
            match writer_g.flush() {
                Ok(()) => (),
                Err(err) => return Err(TrainerError::Io(err)),
            }
        }
        self.algorithm.gradient_adder().start();
        for sample in data {
            match self.algorithm.gradient_adder().intr_checker().check() {
                Ok(()) => (),
                Err(intr) => return Err(TrainerError::Interruption(intr)),
            }
            match sample? {
                Some(sample) => {
                    match self.sampler.samples(&sample) {
                        Some(samples) => {
                            for sample in samples {
                                match minibatches.get_mut(&sample.moves.len()) {
                                    Some(minibatch) => {
                                        if minibatch.is_empty() {
                                            minibatch_count += 1;
                                        }
                                        minibatch.push(sample.clone());
                                    },
                                    None => {
                                        minibatch_count += 1;
                                        minibatches.insert(sample.moves.len(), vec![sample.clone()]);
                                    },
                                }
                                match minibatches.get_mut(&sample.moves.len()) {
                                    Some(minibatch) => {
                                        if self.algorithm.gradient_adder().samples_are_full(minibatch.len()) {
                                            let (tmp_passed_output_count, output_count) = self.algorithm.gradient_adder().compute(minibatch, are_gradients)?;
                                            passed_output_count += tmp_passed_output_count;
                                            all_output_count += output_count;
                                            minibatch.clear();
                                            computed_minibatch_count += 1;
                                            if computed_minibatch_count % Self::MINIBATCH_COUNT_TO_PRINT == 0 {
                                                let mut writer_g = self.writer.lock().unwrap();
                                                match self.printer.print(&mut *writer_g, sample_count, computed_minibatch_count, minibatch_count, false) {
                                                    Ok(()) => (),
                                                    Err(err) => return Err(TrainerError::Io(err)),
                                                }
                                                match writer_g.flush() {
                                                    Ok(()) => (),
                                                    Err(err) => return Err(TrainerError::Io(err)),
                                                }
                                            }
                                        }
                                    },
                                    None => (),
                                }
                            }
                        },
                        None => err_count += 1,
                    }
                },
                None => err_count += 1,
            }
            sample_count += 1;
        }
        for minibatch in minibatches.values_mut() {
            if !minibatch.is_empty() {
                let (tmp_passed_output_count, output_count) = self.algorithm.gradient_adder().compute(minibatch, are_gradients)?;
                passed_output_count += tmp_passed_output_count;
                all_output_count += output_count;
                minibatch.clear();
                computed_minibatch_count += 1;
                if computed_minibatch_count % Self::MINIBATCH_COUNT_TO_PRINT == 0 {
                    let mut writer_g = self.writer.lock().unwrap();
                    match self.printer.print(&mut *writer_g, sample_count, computed_minibatch_count, minibatch_count, false) {
                        Ok(()) => (),
                        Err(err) => return Err(TrainerError::Io(err)),
                    }
                    match writer_g.flush() {
                        Ok(()) => (),
                        Err(err) => return Err(TrainerError::Io(err)),
                    }
                }
            }
        }
        {
            let mut writer_g = self.writer.lock().unwrap();
            match self.printer.print(&mut *writer_g, sample_count, computed_minibatch_count, minibatch_count, true) {
                Ok(()) => (),
                Err(err) => return Err(TrainerError::Io(err)),
            }
            match writer_g.flush() {
                Ok(()) => (),
                Err(err) => return Err(TrainerError::Io(err)),
            }
        }
        Ok((passed_output_count, all_output_count, err_count))
    }
    
    pub fn do_epoch(&self, data: &mut dyn Iterator<Item = TrainerResult<Option<DataSample>>>) -> TrainerResult<(u64, u64, u64)>
    {
        let tuple = self.do_data(data, true)?;
        self.algorithm.gradient_adder().divide();
        self.algorithm.do_alg()?;
        Ok(tuple)
    }

    pub fn do_result(&self, data: &mut dyn Iterator<Item = TrainerResult<Option<DataSample>>>) -> TrainerResult<(u64, u64, u64)>
    { self.do_data(data, false) }
}
