// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app

use casaverde_app::app::{run_app, App};
use casaverde_app::tui::Tui;
use casaverde_utils::fs::read_to_string;
use casaverde_utils::io::{new_error, IoError, IoErrorKind};
use casaverde_utils::log::{error, info, LevelFilter};
use clap::Parser;
use std::path::PathBuf;
use toml::Value;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "https://127.0.0.1:3003")]
    server: String,
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let config_path = PathBuf::from("config.toml");
    info!("Loading config from: {:?}", config_path);
    let config: Value = toml::from_str(&read_to_string(&config_path)?)
        .map_err(|e| new_error(IoErrorKind::Other, format!("TOML parsing error: {}", e)))?;
    info!("Config loaded: {:?}", config);
    let log_level = config.get("logging").and_then(|l| l.get("level")).and_then(|l| l.as_str())
        .map(|s| match s.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        })
        .unwrap_or(LevelFilter::Info);
    casaverde_utils::init_logger("casaverde_app", log_level)?;
    info!("Logger initialized for casaverde_app");

    let args = Args::parse();
    let server = std::env::var("SERVER_IP")
        .map(|ip| format!("https://{ip}"))
        .unwrap_or(args.server);
    info!("Using server: {}", server);

    run_tui_mode(&server, &config_path).await
}

async fn run_tui_mode(server: &str, config_path: &PathBuf) -> Result<(), IoError> {
    let mut tui = Tui::new()?;
    tui.enter()?;
    let mut app = App::new(config_path.to_str().ok_or_else(|| {
        new_error(IoErrorKind::Other, "Invalid config path")
    })?, server);
    if let Err(e) = run_app(&mut tui, &mut app).await {
        error!("Application error: {:?}", e);
    }
    tui.exit()?;
    Ok(())
}
