// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/main.rs

use casaverde_app::app::{run_app, App};
use casaverde_app::tui::Tui;
use casaverde_utils::fs::read_to_string;
use casaverde_utils::io::{new_error, IoError, IoErrorKind};
use casaverde_utils::{init_logger, info, error, LevelFilter, Logger};
use casaverde_utils::path::get_config_path;
use toml::Value;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let config_path = get_config_path("casaverde_app");
    println!("Loading config from: {:?}", config_path);

    let config_str = read_to_string(&config_path)
        .map_err(|e| new_error(IoErrorKind::Other, format!("Failed to read config.toml: {}", e)))?;
    let config: Value = toml::from_str(&config_str)
        .map_err(|e| new_error(IoErrorKind::Other, format!("TOML parsing error: {}", e)))?;

    let log_level = config
        .get("logging")
        .and_then(|l| l.get("level"))
        .and_then(|l| l.as_str())
        .map(|s| match s.to_lowercase().as_str() {
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        })
        .unwrap_or(LevelFilter::Info);

    let mut logger = init_logger("casaverde_app", log_level)?;
    info(&mut logger, &format!("Logger initialized for casaverde_app at {:?}", log_level))?;

    let mut server = "https://127.0.0.1:3003".to_string();
    let cli_args: Vec<String> = std::env::args().collect();
    for i in 1..cli_args.len() {
        if cli_args[i] == "--server" && i + 1 < cli_args.len() {
            server = cli_args[i + 1].clone();
        }
    }
    if let Ok(ip) = std::env::var("SERVER_IP") {
        server = format!("https://{}", ip);
    }
    info(&mut logger, &format!("Using server: {}", server))?;

    info(&mut logger, "Launching TUI mode...")?;
    run_tui_mode(&server, &config_path, &mut logger).await
}

async fn run_tui_mode(server: &str, config_path: &std::path::Path, logger: &mut Logger) -> Result<(), IoError> {
    let mut tui = Tui::new().map_err(|e| new_error(IoErrorKind::Other, format!("Failed to create TUI: {}", e)))?;
    tui.enter().map_err(|e| new_error(IoErrorKind::Other, format!("Failed to enter TUI: {}", e)))?;
    let mut app = App::new(
        config_path.to_str().ok_or_else(|| new_error(IoErrorKind::Other, "Invalid config path"))?,
        server,
        logger,
    );
    if let Err(e) = run_app(&mut tui, &mut app, logger).await {
        error(logger, &format!("Application error: {:?}", e))?;
    }
    tui.exit().map_err(|e| new_error(IoErrorKind::Other, format!("Failed to exit TUI: {}", e)))?;
    Ok(())
}
