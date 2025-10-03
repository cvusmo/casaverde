// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/main.rs

use casaverde_app::app::{run_app, CasaverdeApp};
use casaverde_app::touch::run_touchscreen;
use casaverde_utils;
use clap::Parser;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::info;
use ratatui::backend::CrosstermBackend;
use std::io;
use tokio;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    tui: bool,
    #[arg(long, default_value = "https://10.0.0.6:3003")]
    server: String,
    #[arg(long, default_value_t = false)]
    local_server: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    casaverde_utils::init_logger("casaverde_app", log::LevelFilter::Info)?;
    info!("Starting Casaverde application");

    let args = Args::parse();
    let server = std::env::var("SERVER_IP")
        .map(|ip| format!("https://{ip}:3003"))
        .unwrap_or(args.server);
    if args.tui {
        run_tui(&server).await
    } else {
        run_touchscreen()
    }
}

async fn run_tui(_server: &str) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(LeaveAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;
    let app = CasaverdeApp::new();
    tokio::runtime::Runtime::new()?.block_on(run_app(&mut terminal, app))?;
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    info!("TUI session ended");
    Ok(())
}
