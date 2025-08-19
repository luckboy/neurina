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
use neurina::trainer::algorithms::AdadeltaAlgFactory;
use neurina::trainer::algorithms::AdagradAlgFactory;
use neurina::trainer::algorithms::AdamAlgFactory;
use neurina::trainer::algorithms::ExpSgdAlgFactory;
use neurina::trainer::algorithms::GdAlgFactory;
use neurina::trainer::algorithms::MomentumAlgFactory;
use neurina::trainer::algorithms::PolySgdAlgFactory;
use neurina::trainer::algorithms::RmsPropAlgFactory;
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
    ExpSgd,
    PolySgd,
    Momentum,
    Adagrad,
    RmsProp,
    Adadelta,
    Adam,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
#[clap(rename_all = "kebab_case")]
enum NetworkVersion
{
    V1,
    V2,
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
    /// Stop for percent of passed outputs
    #[arg(short = 'p', long, value_name = "PERCENT")]
    stop_for_percent: Option<u64>,
    /// Network version
    #[arg(short = 'v', long, value_name = "VERSION", value_enum, default_value_t = NetworkVersion::V2)]
    network_version: NetworkVersion,
}

fn initialize_sampler(args: &Args) -> Arc<dyn Sample + Send + Sync>
{
    match args.sampler {
        Sampler::Single => Arc::new(SingleSampler::new()),
        Sampler::Multi => Arc::new(MultiSampler::new()),
    }
}

fn initialize_algorithm_v1(args: &Args) -> Result<Arc<dyn Algorithm + Send + Sync>>
{
    match args.algorithm {
        Alg::Gd => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = GradientAdderFactory::new(NetworkLoader::new(), XavierNetworkFactory::new(args.network_size));
            let alg_factory = GdAlgFactory::new(gradient_adder_factory);
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::ExpSgd => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = GradientAdderFactory::new(NetworkLoader::new(), XavierNetworkFactory::new(args.network_size));
            let alg_factory = ExpSgdAlgFactory::new(gradient_adder_factory);
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::PolySgd => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = GradientAdderFactory::new(NetworkLoader::new(), XavierNetworkFactory::new(args.network_size));
            let alg_factory = PolySgdAlgFactory::new(gradient_adder_factory);
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::Momentum => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = GradientAdderFactory::new(NetworkLoader::new(), XavierNetworkFactory::new(args.network_size));
            let alg_factory = MomentumAlgFactory::new(gradient_adder_factory, NetworkLoader::new(), ZeroNetworkFactory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::Adagrad => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = GradientAdderFactory::new(NetworkLoader::new(), XavierNetworkFactory::new(args.network_size));
            let alg_factory = AdagradAlgFactory::new(gradient_adder_factory, NetworkLoader::new(), ZeroNetworkFactory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::RmsProp => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = GradientAdderFactory::new(NetworkLoader::new(), XavierNetworkFactory::new(args.network_size));
            let alg_factory = RmsPropAlgFactory::new(gradient_adder_factory, NetworkLoader::new(), ZeroNetworkFactory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::Adadelta => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = GradientAdderFactory::new(NetworkLoader::new(), XavierNetworkFactory::new(args.network_size));
            let alg_factory = AdadeltaAlgFactory::new(gradient_adder_factory, NetworkLoader::new(), ZeroNetworkFactory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::Adam => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = GradientAdderFactory::new(NetworkLoader::new(), XavierNetworkFactory::new(args.network_size));
            let alg_factory = AdamAlgFactory::new(gradient_adder_factory, NetworkLoader::new(), ZeroNetworkFactory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
    }
}

fn initialize_algorithm_v2(args: &Args) -> Result<Arc<dyn Algorithm + Send + Sync>>
{
    match args.algorithm {
        Alg::Gd => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = OneGradientAdderFactory::new(NetworkV2Loader::new(), XavierNetworkV2Factory::new(args.network_size));
            let alg_factory = GdAlgFactory::new(gradient_adder_factory);
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::ExpSgd => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = OneGradientAdderFactory::new(NetworkV2Loader::new(), XavierNetworkV2Factory::new(args.network_size));
            let alg_factory = ExpSgdAlgFactory::new(gradient_adder_factory);
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::PolySgd => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = OneGradientAdderFactory::new(NetworkV2Loader::new(), XavierNetworkV2Factory::new(args.network_size));
            let alg_factory = PolySgdAlgFactory::new(gradient_adder_factory);
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::Momentum => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = OneGradientAdderFactory::new(NetworkV2Loader::new(), XavierNetworkV2Factory::new(args.network_size));
            let alg_factory = MomentumAlgFactory::new(gradient_adder_factory, NetworkV2Loader::new(), ZeroNetworkV2Factory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::Adagrad => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = OneGradientAdderFactory::new(NetworkV2Loader::new(), XavierNetworkV2Factory::new(args.network_size));
            let alg_factory = AdagradAlgFactory::new(gradient_adder_factory, NetworkV2Loader::new(), ZeroNetworkV2Factory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::RmsProp => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = OneGradientAdderFactory::new(NetworkV2Loader::new(), XavierNetworkV2Factory::new(args.network_size));
            let alg_factory = RmsPropAlgFactory::new(gradient_adder_factory, NetworkV2Loader::new(), ZeroNetworkV2Factory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::Adadelta => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = OneGradientAdderFactory::new(NetworkV2Loader::new(), XavierNetworkV2Factory::new(args.network_size));
            let alg_factory = AdadeltaAlgFactory::new(gradient_adder_factory, NetworkV2Loader::new(), ZeroNetworkV2Factory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
        Alg::Adam => {
            initialize_ctrl_c_intr_checker();
            let intr_checker = Arc::new(CtrlCIntrChecker::new());
            let converter = Converter::new(IndexConverter::new());
            let gradient_adder_factory = OneGradientAdderFactory::new(NetworkV2Loader::new(), XavierNetworkV2Factory::new(args.network_size));
            let alg_factory = AdamAlgFactory::new(gradient_adder_factory, NetworkV2Loader::new(), ZeroNetworkV2Factory::new(args.network_size));
            Ok(Arc::new(alg_factory.create(intr_checker, converter)?))
        },
    }
}

fn initialize_algorithm(args: &Args) -> Result<Arc<dyn Algorithm + Send + Sync>>
{
    match args.network_version {
        NetworkVersion::V1 => initialize_algorithm_v1(args),
        NetworkVersion::V2 => initialize_algorithm_v2(args),
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

fn perc(x: u64, y: u64) -> u64
{
    if y != 0 {
        (x * 100) / y
    } else {
        0
    }
}

fn print_passed_and_errors(passed_output_count: u64, all_output_count: u64, err_count: u64)
{ println!("passed: {}/{} ({}%), errors: {}", passed_output_count, all_output_count, perc(passed_output_count, all_output_count), err_count); }

fn print_time(s: &str, duration: Duration)
{ println!("{} time: {}:{:02}:{:02}.{:03}", s, (duration.as_secs() / 60) / 60,  (duration.as_secs() / 60) % 60, duration.as_secs() % 60, duration.as_millis() % 1000); }

fn append_passed_gnuplot_data(epoch: usize, passed_output_count: u64, all_output_count: u64, is_result: bool) -> Result<()>
{
    if is_result {
        copy_and_append_gnuplot_data("passed-1.dat", "passed.dat", epoch, passed_output_count)?;
    } else {
        append_gnuplot_data("passed-1.dat", epoch, passed_output_count)?;
    }
    if is_result {
        copy_and_append_gnuplot_data("passed_perc-1.dat", "passed_perc.dat", epoch, perc(passed_output_count, all_output_count))?;
    } else {
        append_gnuplot_data("passed_perc-1.dat", epoch, perc(passed_output_count, all_output_count))?;
    }
    Ok(())
}

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
    if args.network_size == 0 {
        eprintln!("network size is zero");
        exit(1);
    }
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
        let epoch = trainer.epoch();
        println!("epoch: {}", epoch);
        let mut reader = match LichessPuzzleReader::from_path(args.lichess_puzzles.as_str()) {
            Ok(tmp_reader) => tmp_reader,
            Err(err) => {
                eprintln!("{}", err);
                finalize_backend_and_exit(1);
            },
        };
        let mut puzzles = reader.puzzles(args.max_lichess_puzzles);
        let now = Instant::now();
        let (passed_output_count, all_output_count) = match trainer.do_epoch(&mut puzzles) {
            Ok((passed_output_count, all_output_count, err_count)) => {
                print_passed_and_errors(passed_output_count, all_output_count, err_count);
                match append_passed_gnuplot_data(epoch - 1,  passed_output_count, all_output_count, false) {
                    Ok(()) => (),
                    Err(err) => {
                        eprintln!("{}", err);
                        finalize_backend_and_exit(1);
                    },
                }
                (passed_output_count, all_output_count)
            },
            Err(err) => {
                eprintln!("{}", err);
                finalize_backend_and_exit(1);
            },
        };
        print_time("epoch", now.elapsed());
        match trainer.save() {
            Ok(()) => (),
            Err(err) => {
                eprintln!("{}", err);
                finalize_backend_and_exit(1);
            },
        }
        match args.stop_for_percent {
            Some(max_perc) => {
                if perc(passed_output_count, all_output_count) >= max_perc {
                    break;
                }
            },
            None => (),
        }
    }
    if !args.no_result {
        let epoch = trainer.epoch();
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
                print_passed_and_errors(passed_output_count, all_output_count, err_count);
                match append_passed_gnuplot_data(epoch - 1,  passed_output_count, all_output_count, true) {
                    Ok(()) => (),
                    Err(err) => {
                        eprintln!("{}", err);
                        finalize_backend_and_exit(1);
                    },
                }
            },
            Err(err) => {
                eprintln!("{}", err);
                finalize_backend_and_exit(1);
            },
        }
        print_time("result", now.elapsed());
    }
    match finalize_backend() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    }
}
