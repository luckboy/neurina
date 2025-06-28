//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::fs::File;
use std::io::stdout;
use std::process::exit;
use std::sync::Arc;
use std::sync::Mutex;
use clap::Parser;
use neurina::selector::*;
use neurina::shared::*;

#[derive(Parser, Debug)]
#[command(version)]
struct Args
{
    /// Lichess puzzle database file
    #[arg(short, long, value_name = "FILE")]
    lichess_puzzles: String,
    /// Maximal number of puzzles
    #[arg(short, long, value_name = "NUMBER")]
    max_lichess_puzzles: Option<u64>,
    /// Output file
    #[arg(short, long, value_name = "FILE")]
    output: String,
    /// Divide number of lichess puzzles
    #[arg(short, long, value_name = "NUMBER")]
    divider: u64,
}

fn initialize_selector() -> Selector
{
    initialize_ctrl_c_intr_checker();
    let intr_checker = Arc::new(CtrlCIntrChecker::new());
    let writer = Arc::new(Mutex::new(stdout()));
    let printer = Arc::new(Printer::new());
    Selector::new(intr_checker, writer, printer)
}

fn main()
{
    let args = Args::parse();
    if args.divider == 0 {
        eprintln!("divider is zero");
        exit(1);
    }
    let selector = initialize_selector();
    let mut reader = match LichessPuzzleReader::from_path(args.lichess_puzzles.as_str()) {
        Ok(tmp_reader) => tmp_reader,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    };
    let mut puzzles = reader.puzzles(args.max_lichess_puzzles);
    let mut writer = match File::create(args.output) {
        Ok(tmp_writer) => tmp_writer,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    };
    match selector.select(&mut puzzles, &mut writer, args.divider) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        },
    }
}
