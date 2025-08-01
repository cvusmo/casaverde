use axum::{routing::get, Router, response::Json};
use serde::Serialize;
use std::process::Command;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct TempData {
    cpu: Option<f32>,
    gpu: Option<f32>,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/temps", get(get_temperatures));
    let listener = TcpListener::bind("10.0.0.12:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_temperatures() -> Json<TempData> {
    let sensors_output = Command::new("sensors")
        .output()
        .expect("Failed to run sensors");
    let sensors_str = String::from_utf8_lossy(&sensors_output.stdout);
    let cpu_temp = parse_cpu_temp(&sensors_str);

    let nvidia_output = Command::new("nvidia-smi")
        .arg("--query-gpu=temperature.gpu")
        .arg("--format=csv,noheader")
        .output()
        .expect("Failed to run nvidia-smi");
    let nvidia_str = String::from_utf8_lossy(&nvidia_output.stdout);
    let gpu_temp = parse_gpu_temp(&nvidia_str);

    Json(TempData { cpu: cpu_temp, gpu: gpu_temp })
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
    if temp_str.is_empty() {
        return None;
    }
    temp_str.parse::<f32>().ok()
}
