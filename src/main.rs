//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::sync::Arc;
use std::sync::Mutex;
use clap::Parser;
use neurina::matrix::Matrix;
use neurina::engine::*;
use neurina::shared::*;

#[derive(Parser, Debug)]
#[command(version)]
struct Args
{
    /// Configuration file
    #[arg(short, long, value_name = "CONFIG_FILE", default_value_t = String::from("neurina.toml"))]
    config: String,
    /// Network file
    #[arg(short, long, value_name = "NETWORK_FILE", default_value_t = String::from("neurina.nnet"))]
    network: String,
    /// Set random network
    #[arg(long, value_name = "NUMBER")]
    random_network: Option<usize>,
    /// Write logs to log file
    #[arg(short, long, value_name = "LOG_FILE")]
    log: Option<String>,
    /// Load Syzygy endgame tablebases
    #[arg(short, long, value_name = "SYZYGY_PATH")]
    syzygy: Option<String>,
}

const MIDDLE_DEPTH: usize = 2;

fn initialize_engine(args: &Args, config: &Option<Config>, writer: Arc<Mutex<dyn Write + Send + Sync>>, printer: Arc<dyn Print + Send + Sync>) -> LoopResult<Engine>
{
    match initialize_backend(config) {
        Ok(()) => (),
        Err(err) => return Err(LoopError::Matrix(err)),
    }
    let converter = Converter::new(IndexConverter::new());
    let network = match args.random_network {
        Some(count) => {
            let mut iw_elems = vec![0.0f32; count * Converter::BOARD_ROW_COUNT];
            xavier_init(iw_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, count);
            let iw = Matrix::new_with_elems(count, Converter::BOARD_ROW_COUNT, iw_elems.as_slice());
            let mut ib_elems = vec![0.0f32; count];
            xavier_init(ib_elems.as_mut_slice(), Converter::BOARD_ROW_COUNT, count);
            let ib = Matrix::new_with_elems(count, 1, ib_elems.as_slice());
            let mut sw_elems = vec![0.0f32; count * count];
            xavier_init(sw_elems.as_mut_slice(), count, count);
            let sw = Matrix::new_with_elems(count, count, sw_elems.as_slice());
            let mut sb_elems = vec![0.0f32; count];
            xavier_init(sb_elems.as_mut_slice(), count, count);
            let sb = Matrix::new_with_elems(count, 1, sb_elems.as_slice());
            let mut pw_elems = vec![0.0f32; count * count];
            xavier_init(pw_elems.as_mut_slice(), count, count);
            let pw = Matrix::new_with_elems(count, count, pw_elems.as_slice());
            let mut pb_elems = vec![0.0f32; count];
            xavier_init(pb_elems.as_mut_slice(), count, count);
            let pb = Matrix::new_with_elems(count, 1, pb_elems.as_slice());
            let mut ow_elems = vec![0.0f32; converter.move_row_count() * count];
            xavier_init(ow_elems.as_mut_slice(), count, converter.move_row_count());
            let ow = Matrix::new_with_elems(converter.move_row_count(), count, ow_elems.as_slice());
            let mut ob_elems = vec![0.0f32; converter.move_row_count()];
            xavier_init(ob_elems.as_mut_slice(), count, converter.move_row_count());
            let ob = Matrix::new_with_elems(converter.move_row_count(), 1, ob_elems.as_slice());
            Network::new(iw, ib, sw, sb, pw, pb, ow, ob)
        },
        None => {
            match load_network(args.network.as_str()) {
                Ok(tmp_network) => {
                    if !tmp_network.check(Converter::BOARD_ROW_COUNT, converter.move_row_count()) {
                        return Err(LoopError::InvalidNetwork);
                    }
                    tmp_network
                },
                Err(err) => return Err(LoopError::Io(err)),
            }
        },
    };
    let mut config_syzygy_path: Option<String> = None;
    match config {
        Some(config) => {
            match &config.syzygy {
                Some(syzygy) => config_syzygy_path = syzygy.path.clone(),
                None => (),
            }
        },
        None => (),
    }
    let syzygy = match args.syzygy.as_ref().or(config_syzygy_path.as_ref()) {
        Some(syzygy_path) => {
            match Syzygy::new(syzygy_path) {
                Ok(tmp_syzygy) => Arc::new(Mutex::new(Some(tmp_syzygy))),
                Err(err) => return Err(LoopError::Fathom(err)),
            }
        },
        None => Arc::new(Mutex::new(None)),
    };
    let intr_checker = Arc::new(IntrChecker::new());
    let eval_fun = Arc::new(SimpleEvalFun::new());
    let neural_searcher = Arc::new(NeuralSearcher::new(intr_checker, converter, network));
    let middle_searcher = MiddleSearcher::new(eval_fun, neural_searcher);
    let one_searcher = Arc::new(OneSearcher::new(middle_searcher, MIDDLE_DEPTH));
    let thinker = Arc::new(Thinker::new(one_searcher, writer, printer, syzygy));
    Ok(Engine::new(thinker))
}

fn main()
{
    let args = Args::parse();
    let config = match load_config(args.config.as_str()) {
        Ok(tmp_config) => tmp_config,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };
    let stdout_log = match &args.log {
        Some(log_path) => {
            match File::options().create(true).append(true).open(log_path.as_str()) {
                Ok(log_file) => Arc::new(Mutex::new(StdoutLog::new(Some(Box::new(log_file))))),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                },
            }
        },
        None => Arc::new(Mutex::new(StdoutLog::new(None))),
    };
    let mut status = 0;
    match protocol_loop(stdout_log, |writer, printer| initialize_engine(&args, &config, writer, printer)) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            status = 1;
        },
    }
    match finalize_backend() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            status = 1;
        },
    }
    if status != 0 {
        exit(status);
    }
}
