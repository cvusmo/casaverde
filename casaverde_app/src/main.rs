// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/main.rs

use app::{run_app, CasaverdeApp};
use clap::Parser;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::backend::CrosstermBackend;
use std::io::{self, Stdout};
use touch::run_touchscreen;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    tui: bool,
}

// TODO: what if no touchscreen device?
fn main() -> io::Result<()> {
    let args = Args::parse();
    if args.tui {
        run_tui()
    } else {
        run_touchscreen()
    }
}

fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;
    let app = CasaverdeApp::new();
    let res = tokio::runtime::Runtime::new()?.block_on(run_app(&mut terminal, app));
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    res
}
