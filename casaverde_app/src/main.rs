// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/main.rs

use casaverde_app::app::{run_app, App};
use casaverde_app::touch::run_touchscreen;
use casaverde_app::tui::Tui;
use casaverde_utils::init_logger;
use clap::Parser;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use casaverde_utils::log::LevelFilter;
use std::io;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    tui: bool,
    #[arg(long, default_value = "https://127.0.0.1:3003")]
    server: String,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    init_logger("casaverde_app", LevelFilter::Info).expect("Failed to initialize logger");

    let args = Args::parse();
    let server = std::env::var("SERVER_IP")
        .map(|ip| format!("https://{ip}"))
        .unwrap_or(args.server);

    if args.tui {
        run_tui_mode(&server).await
    } else {
        run_touchscreen()
    }
}

async fn run_tui_mode(server: &str) -> io::Result<()> {
    let mut tui = Tui::new()?;             // ✅ create the proper Tui wrapper
    tui.enter()?;                          // ✅ handles alternate screen + raw mode

    let mut app = App::new("config/devices.toml", server);
    if let Err(e) = run_app(&mut tui, &mut app).await {
        eprintln!("Error: {:?}", e);
    }

    tui.exit()?;                           // ✅ handles exit/restore
    Ok(())
}
