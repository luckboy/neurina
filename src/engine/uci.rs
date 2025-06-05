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
use crate::engine::io::*;
use crate::engine::print::*;
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
        write!(w, "info depth {} multipv 1 score cp {} time {} nodes {} nps {}", depth, value, time.as_millis(), node_count, nps)?;
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

fn uci_uciok(stdio_log: &Arc<Mutex<StdioLog>>) -> Result<()>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, "id name Neurina {}", env!("CARGO_PKG_VERSION"))?;
    writeln!(&mut *stdio_log_g, "id author Lukasz Szpakowski")?;
    writeln!(&mut *stdio_log_g, "uciok")?;
    stdio_log_g.flush()?;
    Ok(())
}

fn uci_readyok(stdio_log: &Arc<Mutex<StdioLog>>) -> Result<()>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, "readyok")?;
    stdio_log_g.flush()?;
    Ok(())
}

fn uci_unknown_command(stdio_log: &Arc<Mutex<StdioLog>>, cmd: &str) -> Result<()>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, "Unknown command: {}", cmd)?;
    stdio_log_g.flush()?;
    Ok(())
}

fn initialize_commands(cmds: &mut HashMap<String, fn(&Arc<Mutex<StdioLog>>, &mut Engine, &[&str]) -> Result<bool>>)
{
    cmds.insert(String::from("setoption"), uci_ignore);
    cmds.insert(String::from("ucinewgame"), uci_ucinewgame);
    cmds.insert(String::from("position"), uci_position);
    cmds.insert(String::from("go"), uci_go);
    cmds.insert(String::from("stop"), uci_stop);
    cmds.insert(String::from("ponderhit"), uci_ignore);
    cmds.insert(String::from("quit"), uci_quit);
    cmds.insert(String::from("display"), uci_display);
}

fn uci_ignore(_stdio_log: &Arc<Mutex<StdioLog>>, _engine: &mut Engine, _args: &[&str]) -> Result<bool>
{ Ok(false) }

fn uci_ucinewgame(_stdio_log: &Arc<Mutex<StdioLog>>, engine: &mut Engine, _args: &[&str]) -> Result<bool>
{
    engine.stop();
    engine.do_move_chain(|move_chain| {
            *move_chain = MoveChain::new_initial();
    });
    Ok(false)
}

fn uci_position(_stdio_log: &Arc<Mutex<StdioLog>>, engine: &mut Engine, args: &[&str]) -> Result<bool>
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

fn uci_go(_stdio_log: &Arc<Mutex<StdioLog>>, engine: &mut Engine, args: &[&str]) -> Result<bool>
{
    engine.stop();
    let mut i = 0usize;
    let mut search_moves: Option<Vec<Move>> = None;
    let mut white_time: Option<Duration> = None;
    let mut black_time: Option<Duration> = None;
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
                if i + 1 >= args.len() { break; }
                i += 2;
            },
            Some(arg) if *arg == "binc" => {
                if i + 1 >= args.len() { break; }
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
            let remaining_time = engine.do_move_chain(|move_chain| {
                    match move_chain.last().side() {
                        Color::White => white_time,
                        Color::Black => black_time,
                    }
            });
            match remaining_time {
                Some(remaining_time) => {
                    engine.set_time_control(TimeControl::Mps(0));
                    engine.set_remaining_time(remaining_time);
                    is_timeout = true;
                },
                None => (),
            }
        },
    }
    engine.set_move_count_to_go(move_count_to_go);
    engine.go(search_moves, depth, node_count, move_count_to_checkmate, is_timeout, false, true, true);
    Ok(false)
}

fn uci_stop(_stdio_log: &Arc<Mutex<StdioLog>>, engine: &mut Engine, _args: &[&str]) -> Result<bool>
{
    engine.stop();
    Ok(false)
}

fn uci_quit(_stdio_log: &Arc<Mutex<StdioLog>>, _engine: &mut Engine, _args: &[&str]) -> Result<bool>
{ Ok(true) }

fn uci_display(stdio_log: &Arc<Mutex<StdioLog>>, engine: &mut Engine, _args: &[&str]) -> Result<bool>
{
    engine.stop();
    engine.do_move_chain(|move_chain| {
            let mut stdio_log_g = stdio_log.lock().unwrap();
            write!(&mut *stdio_log_g, "{}",  move_chain.last().pretty(PrettyStyle::Ascii))?;
            writeln!(&mut *stdio_log_g, "{}", move_chain.last().as_fen())?;
            stdio_log_g.flush()?;
            Ok::<(), Error>(())
    })?;
    Ok(false)
}

pub fn uci_loop<F>(stdio_log: Arc<Mutex<StdioLog>>, mut f: F) -> LoopResult<()>
    where F: FnMut(Arc<Mutex<dyn Write + Send + Sync>>, Arc<dyn Print + Send + Sync>) -> LoopResult<Engine>
{
    let mut cmds: HashMap<String, fn(&Arc<Mutex<StdioLog>>, &mut Engine, &[&str]) -> Result<bool>> = HashMap::new();
    let mut err: Option<LoopError> = None;
    let mut engine: Option<Engine> = None;
    initialize_commands(&mut cmds);
    match uci_uciok(&stdio_log) {
        Ok(()) => (),
        Err(err2) => err = Some(LoopError::Io(err2)),
    }
    if err.is_none() {
        loop {
            let mut line = String::new();
            {
                let mut stdio_log_g = stdio_log.lock().unwrap();
                match stdio_log_g.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => (),
                    Err(err2) => {
                        err = Some(LoopError::Io(err2));
                        break;
                    },
                }
            }
            let cmd = str_without_crnl(line.as_str());
            let mut iter = cmd.split_whitespace();            
            match iter.next() {
                Some(cmd_name) => {
                    let args: Vec<&str> = iter.collect();
                    if cmd_name == "debug" {
                        continue;
                    } else if cmd_name == "isready" {
                        match uci_readyok(&stdio_log) {
                            Ok(()) => (),
                            Err(err2) => {
                                err = Some(LoopError::Io(err2));
                                break;
                            },
                        }
                        continue;
                    }
                    if engine.is_none() {
                        match f(stdio_log.clone(), Arc::new(UciPrinter::new())) {
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
                                    match cmd_fun(&stdio_log, engine, args.as_slice()) {
                                        Ok(is_exit) if is_exit => break,
                                        Ok(_) => (),
                                        Err(err2) => {
                                            err = Some(LoopError::Io(err2));
                                            break;
                                        },
                                    }
                                },
                                None => {
                                    match uci_unknown_command(&stdio_log, cmd) {
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
