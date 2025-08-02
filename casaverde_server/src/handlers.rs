// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/handlers.rs

use axum::{response::Json, extract::Json as AxumJson};
use std::process::Command;
use crate::models::{TempData, SensorReading};
use crate::cache::{get_cache, insert_cache};

pub async fn get_temperatures() -> Json<TempData> {
    let cpu_temp = match Command::new("sensors").output() {
        Ok(output) => {
            let sensors_str = String::from_utf8_lossy(&output.stdout);
            parse_cpu_temp(&sensors_str)
        }
        Err(e) => {
            eprintln!("Failed to run sensors: {}", e);
            None
        }
    };

    let gpu_temp = match Command::new("nvidia-smi")
        .arg("--query-gpu=temperature.gpu")
        .arg("--format=csv,noheader")
        .output()
    {
        Ok(output) => {
            let nvidia_str = String::from_utf8_lossy(&output.stdout);
            parse_gpu_temp(&nvidia_str)
        }
        Err(e) => {
            eprintln!("Failed to run nvidia-smi: {}", e);
            None
        }
    };

    Json(TempData { cpu: cpu_temp, gpu: gpu_temp })
}

pub async fn post_sensor_data(AxumJson(data): AxumJson<SensorReading>) {
    insert_cache(data.client_id.clone(), (data.temp_data.clone(), std::time::Instant::now()));
}

pub async fn get_all_data() -> Json<Vec<(String, TempData)>> {
    let cache = get_cache();
    Json(cache.lock().unwrap().iter().map(|(id, (data, _))| (id.clone(), data.clone())).collect())
}

fn parse_cpu_temp(output: &str) -> Option<f32> {
    for line in output.lines() {
        if line.contains("Package id 0") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for part in parts {
                if part.ends_with("°C") {
                    if let Ok(temp) = part.trim_end_matches("°C").trim_start_matches('+').parse::<f32>() {
                        return Some(temp);
                    }
                }
            }
        }
    }
    None
}

fn parse_gpu_temp(output: &str) -> Option<f32> {
    let temp_str = output.trim();
    if temp_str.is_empty() { None } else { temp_str.parse().ok() }
}
