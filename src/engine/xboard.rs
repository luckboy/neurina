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
use crate::chess::types::OutcomeFilter;
use crate::chess::board::PrettyStyle;
use crate::chess::moves::Style;
use crate::chess::Board;
use crate::chess::Color;
use crate::chess::DrawReason;
use crate::chess::Move;
use crate::chess::MoveChain;
use crate::chess::Outcome;
use crate::chess::WinReason;
use crate::engine::engine::*;
use crate::engine::io::*;
use crate::engine::print::*;
use crate::engine::utils::*;
use crate::engine::LoopError;
use crate::engine::LoopResult;

fn color_to_str(color: Color) -> &'static str
{
    match color {
        Color::White => "White",
        Color::Black => "Black",
    }
}

fn write_outcome(w: &mut dyn Write, outcome: Outcome) -> Result<()>
{
    match outcome {
        Outcome::Win { side: Color::White, .. } => write!(w, "1-0 ")?,
        Outcome::Win { side: Color::Black, .. } => write!(w, "0-1 ")?,
        Outcome::Draw(_) => write!(w, "1/2-1/2 ")?,
    }
    match outcome {
        Outcome::Win { side, reason: WinReason::Checkmate, } => writeln!(w, "{{{} mates}}", color_to_str(side))?,
        Outcome::Win { side, reason: WinReason::TimeForfeit, } => writeln!(w, "{{{} forfeits on time}}", color_to_str(side.inv()))?,
        Outcome::Win { side, reason: WinReason::InvalidMove, } => writeln!(w, "{{{} made invalid move}}", color_to_str(side.inv()))?,
        Outcome::Win { side, reason: WinReason::EngineError, } => writeln!(w, "{{{} is buggy engine}}", color_to_str(side.inv()))?,
        Outcome::Win { side, reason: WinReason::Resign, } => writeln!(w, "{{{} resigns}}", color_to_str(side.inv()))?,
        Outcome::Win { side, reason: WinReason::Abandon, } => writeln!(w, "{{{} abandons game}}", color_to_str(side.inv()))?,
        Outcome::Win { side, reason: _, } => writeln!(w, "{{{} wins by unknown reason}}", color_to_str(side))?,
        Outcome::Draw(DrawReason::Stalemate) => write!(w, "{{Stalemeate}}")?,
        Outcome::Draw(DrawReason::InsufficientMaterial) => write!(w, "{{Insufficient material}}")?,
        Outcome::Draw(DrawReason::Moves75) => write!(w, "{{Draw by 75 move rule}}")?,
        Outcome::Draw(DrawReason::Repeat5) => write!(w, "{{Draw by fivefold repetition}}")?,
        Outcome::Draw(DrawReason::Moves50) => write!(w, "{{Draw by 50 move rule}}")?,
        Outcome::Draw(DrawReason::Repeat3) => write!(w, "{{Draw by threefold repetition}}")?,
        Outcome::Draw(DrawReason::Agreement) => write!(w, "{{Draw by agreement}}")?,
        Outcome::Draw(_) => write!(w, "{{Draw by unknown reason}}")?,
    }
    Ok(())
}

#[derive(Copy, Clone, Debug)]
pub struct XboardPrinter;

impl XboardPrinter
{
    pub fn new() -> Self
    { XboardPrinter }
}

impl Print for XboardPrinter
{
    fn print_pv(&self, w: &mut dyn Write, board: &Board, depth: usize, value: i32, time: Duration, node_count: u64, pv: &[Move]) -> Result<()>
    {
        write!(w, "{} {} {} {}", depth, value, time.as_millis() / 10, node_count)?;
        let mut tmp_board = board.clone();
        for mv in pv {
            match mv.styled(&tmp_board, Style::San) {
                Ok(style_move) => {
                    write!(w, " {}", style_move)?;
                    match tmp_board.make_move(*mv) {
                        Ok(tmp_new_board) => tmp_board = tmp_new_board,
                        Err(_) => break,
                    }
                },
                Err(_) => break,
            }
        }
        writeln!(w, "")?;
        Ok(())
    }
    
    fn print_best_move(&self, w: &mut dyn Write, _board: &Board, mv: Move) -> Result<()>
    { writeln!(w, "move {}", mv.uci()) }
    
    fn print_outcome(&self, w: &mut dyn Write, outcome: Outcome) -> Result<()>
    { write_outcome(w, outcome) }
}

fn xboard_protover_for_pre_init(stdio_log: &Arc<Mutex<StdioLog>>) -> Result<()>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, "feature done=0")?;
    stdio_log_g.flush()?;
    Ok(())
}

fn xboard_protover_for_post_init(stdio_log: &Arc<Mutex<StdioLog>>) -> Result<()>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, "feature ping=1")?;
    writeln!(&mut *stdio_log_g, "feature setboard=1")?;
    writeln!(&mut *stdio_log_g, "feature playother=1")?;
    writeln!(&mut *stdio_log_g, "feature time=1")?;
    writeln!(&mut *stdio_log_g, "feature draw=0")?;
    writeln!(&mut *stdio_log_g, "feature sigint=0")?;
    writeln!(&mut *stdio_log_g, "feature sigterm=0")?;
    writeln!(&mut *stdio_log_g, "feature reuse=1")?;
    writeln!(&mut *stdio_log_g, "feature analyze=1")?;
    writeln!(&mut *stdio_log_g, "feature myname=\"Neurina\"")?;
    writeln!(&mut *stdio_log_g, "feature variants=\"normal\"")?;
    writeln!(&mut *stdio_log_g, "feature colors=0")?;
    writeln!(&mut *stdio_log_g, "feature name=0")?;
    writeln!(&mut *stdio_log_g, "feature done=1")?;
    stdio_log_g.flush()?;
    Ok(())
}

struct Context
{
    engine: Engine,
    depth: Option<usize>,
    has_force: bool,
    has_analysis: bool,
    can_print_pv: bool,
    analysis_commands: HashMap<String, (fn(&Arc<Mutex<StdioLog>>, &mut Context, &[&str], &str) -> Result<bool>, Option<usize>, Option<usize>)>,
}

impl Context
{
    fn new(engine: Engine, analysis_commands: HashMap<String, (fn(&Arc<Mutex<StdioLog>>, &mut Context, &[&str], &str) -> Result<bool>, Option<usize>, Option<usize>)>) -> Self
    {
        Context {
            engine,
            depth: None,
            has_force: false,
            has_analysis: false,
            can_print_pv: false,
            analysis_commands,
        }
    }
}

fn xboard_illegal_move(stdio_log: &Arc<Mutex<StdioLog>>, s: &str) -> Result<()>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, "Illegal move: {}", s)?;
    stdio_log_g.flush()?;
    Ok(())
}

fn xboard_error(stdio_log: &Arc<Mutex<StdioLog>>, err_type: &str, cmd: &str) -> Result<()>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, "Error ({}): {}", err_type, cmd)?;
    stdio_log_g.flush()?;
    Ok(())
}

fn xboard_outcome(stdio_log: &Arc<Mutex<StdioLog>>, outcome: Outcome) -> Result<()>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    write_outcome(&mut *stdio_log_g, outcome)?;
    stdio_log_g.flush()?;
    Ok(())
}

fn initialize_commands(cmds: &mut HashMap<String, (fn(&Arc<Mutex<StdioLog>>, &mut Context, &[&str], &str) -> Result<bool>, Option<usize>, Option<usize>)>)
{
    cmds.insert(String::from("accepted"), (xboard_ignore, None, None));
    cmds.insert(String::from("rejected"), (xboard_ignore, None, None));
    cmds.insert(String::from("new"), (xboard_new, Some(0), Some(0)));
    cmds.insert(String::from("quit"), (xboard_quit, Some(0), Some(0)));
    cmds.insert(String::from("force"), (xboard_force, Some(0), Some(0)));
    cmds.insert(String::from("go"), (xboard_go, Some(0), Some(0)));
    cmds.insert(String::from("playother"), (xboard_playother, Some(0), Some(0)));
    cmds.insert(String::from("level"), (xboard_level, Some(3), Some(3)));
    cmds.insert(String::from("st"), (xboard_st, Some(1), Some(1)));
    cmds.insert(String::from("sd"), (xboard_sd, Some(1), Some(1)));
    cmds.insert(String::from("time"), (xboard_time, Some(1), Some(1)));
    cmds.insert(String::from("otim"), (xboard_ignore, Some(1), Some(1)));
    cmds.insert(String::from("?"), (xboard_question, Some(0), Some(0)));
    cmds.insert(String::from("ping"), (xboard_ping, Some(1), Some(1)));
    cmds.insert(String::from("result"), (xboard_ignore, Some(2), None));
    cmds.insert(String::from("setboard"), (xboard_setboard, Some(6), Some(6)));
    cmds.insert(String::from("hint"), (xboard_ignore, Some(0), Some(0)));
    cmds.insert(String::from("bk"), (xboard_bk, Some(0), Some(0)));
    cmds.insert(String::from("undo"), (xboard_undo, Some(0), Some(0)));
    cmds.insert(String::from("remove"), (xboard_remove, Some(0), Some(0)));
    cmds.insert(String::from("hard"), (xboard_ignore, Some(0), Some(0)));
    cmds.insert(String::from("easy"), (xboard_ignore, Some(0), Some(0)));
    cmds.insert(String::from("post"), (xboard_post, Some(0), Some(0)));
    cmds.insert(String::from("nopost"), (xboard_nopost, Some(0), Some(0)));
    cmds.insert(String::from("analyze"), (xboard_analyze, Some(0), Some(0)));
    cmds.insert(String::from("display"), (xboard_display, Some(0), Some(0)));
}

fn initialize_analysis_commands(cmds: &mut HashMap<String, (fn(&Arc<Mutex<StdioLog>>, &mut Context, &[&str], &str) -> Result<bool>, Option<usize>, Option<usize>)>)
{
    cmds.insert(String::from("undo"), (xboard_undo, Some(0), Some(0)));
    cmds.insert(String::from("new"), (xboard_new, Some(0), Some(0)));
    cmds.insert(String::from("setboard"), (xboard_new, Some(6), Some(6)));
    cmds.insert(String::from("exit"), (xboard_exit, Some(0), Some(0)));
    cmds.insert(String::from("."), (xboard_dot, Some(0), Some(0)));
    cmds.insert(String::from("hint"), (xboard_ignore, Some(0), Some(0)));
    cmds.insert(String::from("bk"), (xboard_bk, Some(0), Some(0)));
    cmds.insert(String::from("ping"), (xboard_ping, Some(1), Some(1)));
}

fn xboard_go_for_engine(context: &mut Context)
{ context.engine.go(None, context.depth, None, None, !context.has_analysis, !context.has_analysis, context.can_print_pv, !context.has_analysis); }

fn xboard_ignore(_stdio_log: &Arc<Mutex<StdioLog>>, _context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{ Ok(false) }

fn xboard_new(_stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    if !context.has_analysis {
        context.has_force = false;
    }
    context.engine.do_move_chain(|move_chain| {
            *move_chain = MoveChain::new_initial();
    });
    if context.has_analysis {
        xboard_go_for_engine(context);
    }
    Ok(false)
}

fn xboard_quit(_stdio_log: &Arc<Mutex<StdioLog>>, _context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{ Ok(true) }

fn xboard_force(_stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    context.has_force = true;
    Ok(false)
}

fn xboard_go(_stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    context.has_force = false;
    xboard_go_for_engine(context);
    Ok(false)
}

fn xboard_playother(_stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    context.has_force = false;
    Ok(false)
}

fn xboard_level(stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, args: &[&str], cmd: &str) -> Result<bool>
{
    match args[0].parse::<usize>() {
        Ok(mps) => context.engine.set_time_control(TimeControl::Mps(mps)),
        Err(_) => xboard_error(stdio_log, "invalid number", cmd)?,
    }
    Ok(false)
}

fn xboard_st(stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, args: &[&str], cmd: &str) -> Result<bool>
{
    match args[0].parse::<u64>() {
        Ok(timeout) => context.engine.set_time_control(TimeControl::Fixed(Duration::from_secs(timeout))),
        Err(_) => xboard_error(stdio_log, "invalid number", cmd)?,
    }
    Ok(false)
}

fn xboard_sd(stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, args: &[&str], cmd: &str) -> Result<bool>
{
    match args[0].parse::<usize>() {
        Ok(depth) => context.depth = Some(depth),
        Err(_) => xboard_error(stdio_log, "invalid number", cmd)?,
    }
    Ok(false)
}

fn xboard_time(stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, args: &[&str], cmd: &str) -> Result<bool>
{
    match args[0].parse::<u64>() {
        Ok(remaining_time) => context.engine.set_remaining_time(Duration::from_millis(remaining_time * 10)),
        Err(_) => xboard_error(stdio_log, "invalid number", cmd)?,
    }
    Ok(false)
}

fn xboard_question(_stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    context.engine.stop();
    Ok(false)
}

fn xboard_ping(stdio_log: &Arc<Mutex<StdioLog>>, _context: &mut Context, args: &[&str], _cmd: &str) -> Result<bool>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, "pong {}", args[0])?;
    stdio_log_g.flush()?;
    Ok(false)
}

fn xboard_setboard(stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, args: &[&str], cmd: &str) -> Result<bool>
{
    if context.has_analysis {
        context.engine.stop();
    }
    context.engine.do_move_chain(|move_chain| {
            match Board::from_fen(format!("{} {} {} {} {} {}", args[0], args[1], args[2], args[3], args[4], args[5]).as_str()) {
                Ok(board) => *move_chain = MoveChain::new(board),
                Err(_) => xboard_error(stdio_log, "invalid fen", cmd)?,
            }
            Ok::<(), Error>(())
    })?;
    if context.has_analysis {
        xboard_go_for_engine(context);
    }
    Ok(false)
}

fn xboard_bk(stdio_log: &Arc<Mutex<StdioLog>>, _context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, " ")?;
    writeln!(&mut *stdio_log_g, "")?;
    stdio_log_g.flush()?;
    Ok(false)
}

fn xboard_undo(_stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    if context.has_analysis {
        context.engine.stop();
    }
    context.engine.do_move_chain(|move_chain| {
            move_chain.pop();
    });
    if context.has_analysis {
        xboard_go_for_engine(context);
    }
    Ok(false)
}

fn xboard_remove(_stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    context.engine.do_move_chain(|move_chain| {
            move_chain.pop();
            move_chain.pop();
    });
    Ok(false)
}

fn xboard_post(_stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    context.can_print_pv = true;
    Ok(false)
}

fn xboard_nopost(_stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    context.can_print_pv = false;
    Ok(false)
}

fn xboard_analyze(stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    context.has_analysis = true;
    xboard_go_for_engine(context);
    loop {
        let mut line = String::new();
        {
            let mut stdio_log_g = stdio_log.lock().unwrap();
            stdio_log_g.read_line(&mut line)?;
        }
        let cmd = str_without_crnl(line.as_str());
        let mut iter = cmd.split_whitespace();
        match iter.next() {
            Some(cmd_name) => {
                let args: Vec<&str> = iter.collect();
                match context.analysis_commands.get(&String::from(cmd_name)) {
                    Some((cmd_fun, min_arg_count, max_arg_count)) => {
                        match *min_arg_count {
                            Some(min_arg_count) if args.len() < min_arg_count => {
                                xboard_error(stdio_log, "too few arguments", cmd)?;
                            },
                            _ => (),
                        }
                        match *max_arg_count {
                            Some(max_arg_count) if args.len() > max_arg_count => {
                                xboard_error(stdio_log, "too many arguments", cmd)?;
                            },
                            _ => (),
                        }
                        let is_exit = cmd_fun(stdio_log, context, args.as_slice(), cmd)?;
                        if is_exit {
                            break;
                        }
                    },
                    None => xboard_make_move(stdio_log, context, cmd)?,
                }
            },
            None => (),
        }
    }
    context.has_analysis = false;
    Ok(false)
}

fn xboard_exit(_stdio_log: &Arc<Mutex<StdioLog>>, _context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{ Ok(true) }

fn xboard_dot(stdio_log: &Arc<Mutex<StdioLog>>, _context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    let mut stdio_log_g = stdio_log.lock().unwrap();
    writeln!(&mut *stdio_log_g, "stat01...")?;
    stdio_log_g.flush()?;
    Ok(false)
}

fn xboard_display(stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, _args: &[&str], _cmd: &str) -> Result<bool>
{
    context.engine.do_move_chain(|move_chain| {
            let mut stdio_log_g = stdio_log.lock().unwrap();
            write!(&mut *stdio_log_g, "{}",  move_chain.last().pretty(PrettyStyle::Ascii))?;
            writeln!(&mut *stdio_log_g, "{}", move_chain.last().as_fen())?;
            stdio_log_g.flush()?;
            Ok::<(), Error>(())
    })?;
    Ok(false)
}

fn xboard_make_move(stdio_log: &Arc<Mutex<StdioLog>>, context: &mut Context, s: &str) -> Result<()>
{
    if context.has_analysis {
        context.engine.stop();
    }
    context.engine.do_move_chain(|move_chain| {
            let mv = match Move::from_uci_legal(s, move_chain.last()) {
                Ok(tmp_mv) => tmp_mv,
                Err(_) => {
                    match Move::from_san(s, move_chain.last()) {
                        Ok(tmp_mv) => tmp_mv,
                        Err(_) => return xboard_illegal_move(stdio_log, s),
                    }
                },
            };
            match move_chain.push(mv) {
                Ok(()) => (),
                Err(_) => return xboard_illegal_move(stdio_log, s),
            }
            let outcome = move_chain.set_auto_outcome(OutcomeFilter::Relaxed);
            move_chain.clear_outcome();
            match outcome {
                Some(outcome) => xboard_outcome(stdio_log, outcome)?,
                None => (),
            }
            Ok(())
    })?;
    if !context.has_force || context.has_analysis {
        xboard_go_for_engine(context);
    }
    Ok(())
}

pub fn xboard_loop<F>(stdio_log: Arc<Mutex<StdioLog>>, mut f: F) -> LoopResult<()>
    where F: FnMut(Arc<Mutex<StdioLog>>, Arc<dyn Print>) -> LoopResult<Engine>
{
    let mut cmds: HashMap<String, (fn(&Arc<Mutex<StdioLog>>, &mut Context, &[&str], &str) -> Result<bool>, Option<usize>, Option<usize>)> = HashMap::new();
    let mut analysis_cmds: HashMap<String, (fn(&Arc<Mutex<StdioLog>>, &mut Context, &[&str], &str) -> Result<bool>, Option<usize>, Option<usize>)> = HashMap::new();
    let mut err: Option<LoopError> = None;
    let mut context: Option<Context> = None;
    initialize_commands(&mut cmds);
    initialize_analysis_commands(&mut analysis_cmds);
    loop {
        let mut line = String::new();
        {
            let mut stdio_log_g = stdio_log.lock().unwrap();
            match stdio_log_g.read_line(&mut line) {
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
                if cmd_name == "protover" {
                    {
                        match xboard_protover_for_pre_init(&stdio_log) {
                            Ok(_) => (),
                            Err(err2) => {
                                err = Some(LoopError::Io(err2));
                                break;
                            },
                        }
                    }
                    if context.is_none() {
                        match f(stdio_log.clone(), Arc::new(XboardPrinter::new())) {
                            Ok(engine) => context = Some(Context::new(engine, analysis_cmds.clone())),
                            Err(err2) => {
                                err = Some(err2);
                                break;
                            },
                        }
                    }
                    {
                        match xboard_protover_for_post_init(&stdio_log) {
                            Ok(_) => (),
                            Err(err2) => {
                                err = Some(LoopError::Io(err2));
                                break;
                            },
                        }
                    }
                } else if context.is_none() {
                    match f(stdio_log.clone(), Arc::new(XboardPrinter::new())) {
                        Ok(engine) => context = Some(Context::new(engine, analysis_cmds.clone())),
                        Err(err2) => {
                            err = Some(err2);
                            break;
                        },
                    }
                }
                match &mut context {
                    Some(context) => {
                        match cmds.get(&String::from(cmd_name)) {
                            Some((cmd_fun, min_arg_count, max_arg_count)) => {
                                match *min_arg_count {
                                    Some(min_arg_count) if args.len() < min_arg_count => {
                                        match xboard_error(&stdio_log, "too few arguments", cmd) {
                                            Ok(()) => (),
                                            Err(err2) => {
                                                err = Some(LoopError::Io(err2));
                                                break;
                                            },
                                        }
                                    },
                                    _ => (),
                                }
                                match *max_arg_count {
                                    Some(max_arg_count) if args.len() > max_arg_count => {
                                        match xboard_error(&stdio_log, "too many arguments", cmd) {
                                            Ok(()) => (),
                                            Err(err2) => {
                                                err = Some(LoopError::Io(err2));
                                                break;
                                            },
                                        }
                                    },
                                    _ => (),
                                }
                                match cmd_fun(&stdio_log, context, args.as_slice(), cmd) {
                                    Ok(is_exit) if is_exit => break,
                                    Ok(_) => (),
                                    Err(err2) => {
                                        err = Some(LoopError::Io(err2));
                                        break;
                                    },
                                }
                            },
                            None => {
                                match xboard_make_move(&stdio_log, context, cmd) {
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
    match context {
        Some(context) => {
            context.engine.quit();
            context.engine.join_thread();
        },
        None => (),
    }
    match err {
        Some(err) => Err(err),
        None => Ok(()),
    }
}
