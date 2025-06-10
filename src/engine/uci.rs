//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashMap;
use std::io::Error;
use std::io::Result;
use std::io::Write;
use std::io::stdin;
use std::mem::swap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use crate::chess::board::PrettyStyle;
use crate::chess::Board;
use crate::chess::Color;
use crate::chess::Move;
use crate::chess::MoveChain;
use crate::chess::Outcome;
use crate::engine::engine::*;
use crate::engine::engine_id::*;
use crate::engine::io::*;
use crate::engine::print::*;
use crate::engine::syzygy::*;
use crate::engine::utils::*;
use crate::engine::LoopError;
use crate::engine::LoopResult;

#[derive(Copy, Clone, Debug)]
pub struct UciPrinter;

impl UciPrinter
{
    pub fn new() -> Self
    { UciPrinter }
}

impl Print for UciPrinter
{
    fn print_pv(&self, w: &mut dyn Write, _board: &Board, depth: usize, value: i32, time: Duration, node_count: u64, pv: &[Move]) -> Result<()>
    {
        let nps_millis = if time.as_millis() > 0 { time.as_millis() } else { 1 };
        let nps = ((node_count as u128) * 1000) / nps_millis;
        write!(w, "info depth {} multipv 1 score cp {} time {} nodes {} nps {} pv", depth, value, time.as_millis(), node_count, nps)?;
        for mv in pv {
            write!(w, " {}", mv.uci())?;
        }
        writeln!(w, "")?;
        Ok(())
    }
    
    fn print_best_move(&self, w: &mut dyn Write, _board: &Board, mv: Move) -> Result<()>
    { writeln!(w, "bestmove {}", mv.uci()) }
    
    fn print_outcome(&self, _w: &mut dyn Write, _outcome: Outcome) -> Result<()>
    { Ok(()) }
}

fn uci_uciok(stdout_log: &Arc<Mutex<StdoutLog>>, engine_id: EngineId) -> Result<()>
{
    let mut stdout_log_g = stdout_log.lock().unwrap();
    writeln!(&mut *stdout_log_g, "id name {}", engine_id.name)?;
    let mut author = String::new();
    match engine_id.first_author {
        Some(first_author) => {
            author.push_str(first_author);
            author.push_str(" & ");
        },
        None => (),
    }
    author.push_str("Lukasz Szpakowski");
    match engine_id.last_author {
        Some(last_author) => {
            author.push_str(" & ");
            author.push_str(last_author);
        },
        None => (),
    }
    writeln!(&mut *stdout_log_g, "id author {}", author)?;
    writeln!(&mut *stdout_log_g, "option name SyzygyPath type string default ")?;
    writeln!(&mut *stdout_log_g, "uciok")?;
    stdout_log_g.flush()?;
    Ok(())
}

fn uci_readyok(stdout_log: &Arc<Mutex<StdoutLog>>) -> Result<()>
{
    let mut stdout_log_g = stdout_log.lock().unwrap();
    writeln!(&mut *stdout_log_g, "readyok")?;
    stdout_log_g.flush()?;
    Ok(())
}

fn uci_unknown_command(stdout_log: &Arc<Mutex<StdoutLog>>, cmd: &str) -> Result<()>
{
    let mut stdout_log_g = stdout_log.lock().unwrap();
    writeln!(&mut *stdout_log_g, "Unknown command: {}", cmd)?;
    stdout_log_g.flush()?;
    Ok(())
}

fn initialize_commands(cmds: &mut HashMap<String, fn(&Arc<Mutex<StdoutLog>>, &mut Engine, &[&str]) -> Result<bool>>)
{
    cmds.insert(String::from("setoption"), uci_setoption);
    cmds.insert(String::from("ucinewgame"), uci_ucinewgame);
    cmds.insert(String::from("position"), uci_position);
    cmds.insert(String::from("go"), uci_go);
    cmds.insert(String::from("stop"), uci_stop);
    cmds.insert(String::from("ponderhit"), uci_ignore);
    cmds.insert(String::from("quit"), uci_quit);
    cmds.insert(String::from("display"), uci_display);
}

fn uci_ignore(_stdout_log: &Arc<Mutex<StdoutLog>>, _engine: &mut Engine, _args: &[&str]) -> Result<bool>
{ Ok(false) }

fn uci_setoption(_stdout_log: &Arc<Mutex<StdoutLog>>, engine: &mut Engine, args: &[&str]) -> Result<bool>
{
    let mut is_first = true;
    let mut i = 0usize;
    let mut name = String::new();
    let mut value = String::new();
    match args.get(i) {
        Some(arg) if *arg == "name" => {
            i += 1;
            loop {
                match args.get(i) {
                    Some(s) if *s == "value" => break,
                    Some(s) => {
                        if !is_first {
                            name.push(' ');
                        }
                        name.push_str(*s);
                        i += 1;
                        is_first = false;
                    },
                    None => break,
                }
            }
        },
        _ => return Ok(false),
    }
    is_first = true;
    match args.get(i) {
        Some(arg) if *arg == "value" => {
            i += 1;
            loop {
                match args.get(i) {
                    Some(s) => {
                        if !is_first {
                            value.push(' ');
                        }
                        value.push_str(*s);
                        i += 1;
                        is_first = false;
                    },
                    None => break,
                }
            }
        },
        _ => (),
    }
    if name == String::from("SyzygyPath") {
        let mut syzygy_g = engine.thinker().syzygy().lock().unwrap();
        if !value.is_empty() {
            let mut syzygy: Option<Syzygy> = None;
            swap(&mut *syzygy_g, &mut syzygy);
            match syzygy {
                Some(syzygy) => {
                    match syzygy.reload(value) {
                        Ok(tmp_syzygy) => *syzygy_g = Some(tmp_syzygy),
                        Err(_) => (),
                    }
                },
                None => {
                    match Syzygy::new(value) {
                        Ok(tmp_syzygy) => *syzygy_g = Some(tmp_syzygy),
                        Err(_) => (),
                    }
                },
            }
        } else {
            *syzygy_g = None;
        }
    }
    Ok(false)
}

fn uci_ucinewgame(_stdout_log: &Arc<Mutex<StdoutLog>>, engine: &mut Engine, _args: &[&str]) -> Result<bool>
{
    engine.stop();
    engine.do_move_chain(|move_chain| {
            *move_chain = MoveChain::new_initial();
    });
    Ok(false)
}

fn uci_position(_stdout_log: &Arc<Mutex<StdoutLog>>, engine: &mut Engine, args: &[&str]) -> Result<bool>
{
    engine.stop();
    engine.do_move_chain(|move_chain| {
            let mut i = 0usize;
            match args.get(i) {
                Some(arg) if *arg == "startpos" => {
                    *move_chain = MoveChain::new_initial();
                    i += 1;
                },
                Some(arg) if *arg == "fen" && args.len() >= i + 5 => {
                    let mut n = 5;
                    let arg3 = args[i + 3].replace("A", "Q").replace("H", "K").replace("a", "q").replace("h", "k");
                    let mut fen = format!("{} {} {} {}", args[i + 1], args[i + 2], arg3, args[i + 4]);
                    if args.len() >= i + 6 && args[i + 5] != "moves" {
                        fen.push_str(format!(" {}", args[i + 5]).as_str());
                        n = 6;
                        if args.len() >= i + 7 && args[i + 6] != "moves" {
                            fen.push_str(format!(" {}", args[i + 6]).as_str());
                            n = 7;
                        }
                    }
                    match Board::from_fen(fen.as_str()) {
                        Ok(board) => *move_chain = MoveChain::new(board),
                        Err(_) => return,
                    }
                    i += n;
                },
                _ => return,
            }
            match args.get(i) {
                Some(arg) if *arg == "moves" => {
                    for s in &args[(i + 1)..] {
                        match Move::from_uci_legal(s, move_chain.last()) {
                            Ok(mv) => {
                                match move_chain.push(mv) {
                                    Ok(()) => (),
                                    Err(_) => return,
                                }
                            },
                            Err(_) => return,
                        }
                    }
                },
                _ => return,
            }
    });
    Ok(false)
}

fn uci_go(_stdout_log: &Arc<Mutex<StdoutLog>>, engine: &mut Engine, args: &[&str]) -> Result<bool>
{
    engine.stop();
    let mut i = 0usize;
    let mut search_moves: Option<Vec<Move>> = None;
    let mut white_time: Option<Duration> = None;
    let mut black_time: Option<Duration> = None;
    let mut white_inc: Option<Duration> = None;
    let mut black_inc: Option<Duration> = None;
    let mut move_count_to_go = 0usize;
    let mut depth: Option<usize> = None;
    let mut node_count: Option<u64> = None;
    let mut move_count_to_checkmate: Option<usize> = None;
    let mut move_time: Option<Duration> = None;
    loop {
        match args.get(i) {
            Some(arg) if *arg == "searchmoves" => {
                engine.do_move_chain(|move_chain| {
                        i += 1;
                        let mut new_search_moves: Vec<Move> = Vec::new();
                        loop {
                            match args.get(i) {
                                Some(s) => {
                                    match Move::from_uci_legal(s, move_chain.last()) {
                                        Ok(mv) => {
                                            new_search_moves.push(mv);
                                            i += 1;
                                        },
                                        Err(_) => break,
                                    }
                                },
                                None => break,
                            }
                        }
                        search_moves = Some(new_search_moves);
                });
            },
            Some(arg) if *arg == "wtime" => {
                match args.get(i + 1) {
                    Some(s) => white_time = Some(Duration::from_millis(s.parse::<u64>().unwrap_or(0))),
                    None => break,
                }
                i += 2;
            },
            Some(arg) if *arg == "btime" => {
                match args.get(i + 1) {
                    Some(s) => black_time = Some(Duration::from_millis(s.parse::<u64>().unwrap_or(0))),
                    None => break,
                }
                i += 2;
            },
            Some(arg) if *arg == "winc" => {
                match args.get(i + 1) {
                    Some(s) => white_inc = Some(Duration::from_millis(s.parse::<u64>().unwrap_or(0))),
                    None => break,
                }
                i += 2;
            },
            Some(arg) if *arg == "binc" => {
                match args.get(i + 1) {
                    Some(s) => black_inc = Some(Duration::from_millis(s.parse::<u64>().unwrap_or(0))),
                    None => break,
                }
                i += 2;
            },
            Some(arg) if *arg == "movestogo" => {
                match args.get(i + 1) {
                    Some(s) => move_count_to_go = s.parse::<usize>().unwrap_or(0),
                    None => break,
                }
                i += 2;
            },
            Some(arg) if *arg == "depth" => {
                match args.get(i + 1) {
                    Some(s) => depth = Some(s.parse::<usize>().unwrap_or(0)),
                    None => break,
                }
                i += 2;
            },
            Some(arg) if *arg == "nodes" => {
                match args.get(i + 1) {
                    Some(s) => node_count = Some(s.parse::<u64>().unwrap_or(0)),
                    None => break,
                }
                i += 2;
            },
            Some(arg) if *arg == "mate" => {
                match args.get(i + 1) {
                    Some(s) => move_count_to_checkmate = Some(s.parse::<usize>().unwrap_or(0)),
                    None => break,
                }
                i += 2;
            },
            Some(arg) if *arg == "movetime" => {
                match args.get(i + 1) {
                    Some(s) => move_time = Some(Duration::from_millis(s.parse::<u64>().unwrap_or(0))),
                    None => break,
                }
                i += 2;
            },
            Some(_) => i += 1,
            None => break,
        }
    }
    let mut is_timeout = false;
    match move_time {
        Some(move_time) => {
            engine.set_time_control(TimeControl::Fixed(move_time));
            is_timeout = true;
        },
        None => {
            let (remaining_time, inc) = engine.do_move_chain(|move_chain| {
                    match move_chain.last().side() {
                        Color::White => (white_time, white_inc),
                        Color::Black => (black_time, black_inc),
                    }
            });
            match (remaining_time, inc) {
                (Some(remaining_time), Some(inc)) => {
                    engine.set_time_control(TimeControl::Level(0, inc));
                    engine.set_remaining_time(remaining_time);
                    is_timeout = true;
                },
                (Some(remaining_time), None) => {
                    engine.set_time_control(TimeControl::Level(0, Duration::ZERO));
                    engine.set_remaining_time(remaining_time);
                    is_timeout = true;
                },
                (None, _) => (),
            }
        },
    }
    engine.set_move_count_to_go(move_count_to_go);
    engine.go(search_moves, depth, node_count, move_count_to_checkmate, is_timeout, false, true, true);
    Ok(false)
}

fn uci_stop(_stdout_log: &Arc<Mutex<StdoutLog>>, engine: &mut Engine, _args: &[&str]) -> Result<bool>
{
    engine.stop();
    Ok(false)
}

fn uci_quit(_stdout_log: &Arc<Mutex<StdoutLog>>, _engine: &mut Engine, _args: &[&str]) -> Result<bool>
{ Ok(true) }

fn uci_display(stdout_log: &Arc<Mutex<StdoutLog>>, engine: &mut Engine, _args: &[&str]) -> Result<bool>
{
    engine.stop();
    engine.do_move_chain(|move_chain| {
            let mut stdout_log_g = stdout_log.lock().unwrap();
            write!(&mut *stdout_log_g, "{}",  move_chain.last().pretty(PrettyStyle::Ascii))?;
            writeln!(&mut *stdout_log_g, "{}", move_chain.last().as_fen())?;
            stdout_log_g.flush()?;
            Ok::<(), Error>(())
    })?;
    Ok(false)
}

pub fn uci_loop_with_engine_id<F>(stdout_log: Arc<Mutex<StdoutLog>>, engine_id: EngineId, mut f: F) -> LoopResult<()>
    where F: FnMut(Arc<Mutex<dyn Write + Send + Sync>>, Arc<dyn Print + Send + Sync>) -> LoopResult<Engine>
{
    let mut cmds: HashMap<String, fn(&Arc<Mutex<StdoutLog>>, &mut Engine, &[&str]) -> Result<bool>> = HashMap::new();
    let mut err: Option<LoopError> = None;
    let mut engine: Option<Engine> = None;
    initialize_commands(&mut cmds);
    match uci_uciok(&stdout_log, engine_id) {
        Ok(()) => (),
        Err(err2) => err = Some(LoopError::Io(err2)),
    }
    if err.is_none() {
        loop {
            let mut line = String::new();
            match stdin().read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => (),
                Err(err2) => {
                    err = Some(LoopError::Io(err2));
                    break;
                },
            }
            {
                let mut stdout_log_g = stdout_log.lock().unwrap();
                match stdout_log_g.log_input_line(line.as_str()) {
                    Ok(()) => (),
                    Err(err2) => {
                        err = Some(LoopError::Io(err2));
                        break;
                    },
                }
            }
            let cmd = str_without_crnl(line.as_str());
            let trimmed_cmd = cmd.trim();
            let mut iter = trimmed_cmd.split_whitespace();
            match iter.next() {
                Some(cmd_name) => {
                    let args: Vec<&str> = iter.collect();
                    if cmd_name == "debug" {
                        continue;
                    } else if cmd_name == "isready" {
                        match uci_readyok(&stdout_log) {
                            Ok(()) => (),
                            Err(err2) => {
                                err = Some(LoopError::Io(err2));
                                break;
                            },
                        }
                        continue;
                    }
                    if engine.is_none() {
                        match f(stdout_log.clone(), Arc::new(UciPrinter::new())) {
                            Ok(new_engine) => engine = Some(new_engine),
                            Err(err2) => {
                                err = Some(err2);
                                break;
                            },
                        }
                    }
                    match &mut engine {
                        Some(engine) => {
                            match cmds.get(&String::from(cmd_name)) {
                                Some(cmd_fun) => {
                                    match cmd_fun(&stdout_log, engine, args.as_slice()) {
                                        Ok(is_exit) if is_exit => break,
                                        Ok(_) => (),
                                        Err(err2) => {
                                            err = Some(LoopError::Io(err2));
                                            break;
                                        },
                                    }
                                },
                                None => {
                                    match uci_unknown_command(&stdout_log, cmd) {
                                        Ok(()) => (),
                                        Err(err2) => {
                                            err = Some(LoopError::Io(err2));
                                            break;
                                        },
                                    }
                                },
                            }
                        },
                        None => {
                            err = Some(LoopError::UninitializedLoopContext);
                            break;
                        },
                    }
                },
                None => (),
            }
        }
    }
    match engine {
        Some(engine) => {
            engine.quit();
            engine.join_thread();
        },
        None => (),
    }
    match err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}

pub fn uci_loop<F>(stdout_log: Arc<Mutex<StdoutLog>>, f: F) -> LoopResult<()>
    where F: FnMut(Arc<Mutex<dyn Write + Send + Sync>>, Arc<dyn Print + Send + Sync>) -> LoopResult<Engine>
{ uci_loop_with_engine_id(stdout_log, NEURINA_ID, f) }
