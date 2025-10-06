// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_sim

use casaverde_sim::sim::{run_simulation, Cell};
use casaverde_utils::fs::{create_file, read_to_string, write_all};
use casaverde_utils::io::{new_error, IoError, IoErrorKind};
use casaverde_utils::log::{info, LevelFilter};
use casaverde_utils::init_logger;
use serde::Serialize;
use tokio::sync::mpsc;
use toml::Value;
use std::path::PathBuf;

#[derive(Serialize)]
struct CellOutput {
    moisture: f32,
    nutrients: f32,
    plant_height: f32,
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let config_path = PathBuf::from("config.toml");
    info!("Starting casaverde_sim with config: {:?}", config_path);
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
    init_logger("casaverde_sim", log_level)?;
    info!("Logger initialized for casaverde_sim");

    let width = config.get("simulation").and_then(|s| s.get("width")).and_then(|w| w.as_integer()).unwrap_or(10) as usize;
    let height = config.get("simulation").and_then(|s| s.get("height")).and_then(|h| h.as_integer()).unwrap_or(10) as usize;

    let (tx, mut rx) = mpsc::channel::<Vec<Cell>>(100);
    tokio::spawn(run_simulation(tx, width, height));

    let output_file = PathBuf::from("/tmp/casaverde_sim_data.json");
    info!("Writing simulation data to: {:?}", output_file);
    loop {
        if let Some(cells) = rx.recv().await {
            let output: Vec<CellOutput> = cells.iter().map(|c| CellOutput {
                moisture: c.moisture,
                nutrients: c.nutrients,
                plant_height: c.plant_height,
            }).collect();
            let json = serde_json::to_string(&output)
                .map_err(|e| new_error(IoErrorKind::Other, format!("JSON serialization error: {}", e)))?;
            let mut file = create_file(&output_file)
                .map_err(|e| new_error(IoErrorKind::Other, format!("File creation error: {}", e)))?;
            write_all(&mut file, json.as_bytes())
                .map_err(|e| new_error(IoErrorKind::Other, format!("File write error: {}", e)))?;
            info!("Wrote simulation data: {} cells", output.len());
        }
    }
}
