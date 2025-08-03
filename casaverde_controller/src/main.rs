// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/main.rs

use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Duration;
use log::{info, error};
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    server: String,
    controller_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceReading {
    id: String,
    value: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedData {
    client_id: String,
    devices: Vec<DeviceReading>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting casaverde_controller on {}", get_hostname());

    let config_str = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    let client = Client::new();

    loop {
        let resp = client.get(format!("{}/temps", config.server))
            .send()
            .await?
            .json::<Vec<CachedData>>()
            .await?;

        for data in resp {
            for device in &data.devices {
                if let Some(temp) = device.value {
                    if temp > 50.0 {
                        info!("Temperature above 50C for {} on {}: {}. Turn on cooling.", device.id, data.client_id, temp);
                        println!("Simulated command: turn_on_cooling_{} for {}", device.id, data.client_id);
                        // TODO: ADD GPIO relay control here
                    } else {
                        info!("Temperature below 50C for {} on {}: {}. Cooling off.", device.id, data.client_id, temp);
                        println!("Simulated command: turn_off_cooling{} for {}", device.id, data.client_id);
                        // TODO: write algorithm to optimize cooling based off ideal energy
                        // consumption rate (kWh)
                        // EXAMPLE: Set goal of 500 kWh over a 30 day period 
                    }
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

fn get_hostname() -> String {
    std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
}
