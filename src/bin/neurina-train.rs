//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::env::set_current_dir;
use std::io::Result;
use std::io::stdout;
use std::process::exit;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use clap::Parser;
use clap::ValueEnum;
use neurina::shared::*;
use neurina::trainer::algorithms::GdAlgFactory;
use neurina::trainer::*;

#[derive(ValueEnum, Copy, Clone, Debug)]
#[clap(rename_all = "kebab_case")]
enum Sampler
{
    Single,
    Multi,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
#[clap(rename_all = "kebab_case")]
enum Alg
{
    Gd,
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args
{
    /// Configuration file
    #[arg(short, long, value_name = "CONFIG_FILE", default_value_t = String::from("neurina.toml"))]
    config: String,
    /// Change direactory
    #[arg(short, long, value_name = "DIRECTORY")]
    dir: Option<String>,
    /// Number of epochs
    #[arg(short, long)]
    epochs: usize,
    /// Use sampler
    #[arg(short, long, value_enum, default_value_t = Sampler::Multi)]
    sampler: Sampler,
    /// Use algorithm
    #[arg(short, long, value_enum, default_value_t = Alg::Gd)]
    algorithm: Alg,
    /// Lichess puzzle database file
    #[arg(short, long, value_name = "FILE")]
    lichess_puzzles: String,
    /// Maximal number of puzzles
    #[arg(short, long, value_name = "NUMBER")]
    max_lichess_puzzles: Option<u64>,
    /// Network size
    #[arg(short, long)]
    network_size: usize,
    /// Don't print result
    #[arg(long)]
    no_result: bool,
}

fn initialize_sampler(args: &Args) -> Arc<dyn Sample + Send + Sync>
{
    match args.sampler {
        Sampler::Single => Arc::new(SingleSampler::new()),
        Sampler::Multi => Arc::new(MultiSampler::new()),
    }
}

fn initialize_algorithm(args: &Args) -> Result<Arc<dyn Algorithm + Send + Sync>>
{
    match args.algorithm {
        Alg::Gd => {
            initialize_intr_checker();
            let intr_checker = Arc::new(IntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = GradientAdderFactory::new(NetworkLoader::new(), XavierNetworkFactory::new(args.network_size));
            let alg_factory = GdAlgFactory::new(gradient_adder_factory);
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
    }
}

fn initialize_trainer(args: &Args) -> Result<Trainer>
{
    let sampler = initialize_sampler(args);
    let alg = initialize_algorithm(args)?;
    let writer = Arc::new(Mutex::new(stdout()));
    let printer = Arc::new(Printer::new());
    Ok(Trainer::new(sampler, alg, writer, printer))
}

fn print_duration(s: &str, duration: Duration)
{ println!("{} time: {}:{}:{}.{:03}", s, (duration.as_secs() / 60) / 60,  (duration.as_secs() / 60) % 60, duration.as_secs() % 60, duration.as_millis() % 1000); }

fn finalize_backend_and_exit(status: i32) -> !
{
    match finalize_backend() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    }
    exit(status)
}

fn main()
{
    let args = Args::parse();
    let config = match load_config(args.config.as_str()) {
        Ok(tmp_config) => tmp_config,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    };
    match &args.dir {
        Some(dir) => {
            match set_current_dir(dir.as_str()) {
                Ok(()) => (),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                },
            }
        },
        None => (),
    }
    match initialize_backend(&config) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    }
    let trainer = match initialize_trainer(&args) {
        Ok(tmp_trainer) => tmp_trainer,
        Err(err) => {
            eprintln!("{}", err);
            finalize_backend_and_exit(1);
        },
    };
    for _ in 0..args.epochs {
        println!("epoch: {}", trainer.epoch());
        let mut reader = match LichessPuzzleReader::from_path(args.lichess_puzzles.as_str()) {
            Ok(tmp_reader) => tmp_reader,
            Err(err) => {
                eprintln!("{}", err);
                finalize_backend_and_exit(1);
            },
        };
        let mut puzzles = reader.puzzles(args.max_lichess_puzzles);
        let now = Instant::now();
        match trainer.do_epoch(&mut puzzles) {
            Ok((passed_output_count, all_output_count, err_count)) => {
                println!("passed: {}/{}, errors: {}", passed_output_count, all_output_count, err_count);
            },
            Err(err) => {
                eprintln!("{}", err);
                finalize_backend_and_exit(1);
            },
        }
        print_duration("epoch", now.elapsed());
        match trainer.save() {
            Ok(()) => (),
            Err(err) => {
                eprintln!("{}", err);
                finalize_backend_and_exit(1);
            },
        }
    }
    if !args.no_result {
        println!("result");
        let mut reader = match LichessPuzzleReader::from_path(args.lichess_puzzles.as_str()) {
            Ok(tmp_reader) => tmp_reader,
            Err(err) => {
                eprintln!("{}", err);
                finalize_backend_and_exit(1);
            },
        };
        let mut puzzles = reader.puzzles(args.max_lichess_puzzles);
        let now = Instant::now();
        match trainer.do_result(&mut puzzles) {
            Ok((passed_output_count, all_output_count, err_count)) => {
                println!("passed: {}/{}, errors: {}", passed_output_count, all_output_count, err_count);
            },
            Err(err) => {
                eprintln!("{}", err);
                finalize_backend_and_exit(1);
            },
        }
        print_duration("resut", now.elapsed());
    }
    match finalize_backend() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    }
}
