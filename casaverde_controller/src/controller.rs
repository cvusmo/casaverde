// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/controller.rs - Data processing and command generation

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceReading {
    pub id: String,
    pub value: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedData {
    pub client_id: String,
    pub devices: Vec<DeviceReading>,
}

#[derive(Debug)]
pub enum Command {
    TurnOnCooling(String),
    TurnOffCooling(String),
}

// TODO: create algorithm to update process_reading based off cached values and analyzing the data
// this algorithm will be done by the server, not by the controller.
pub fn process_readings(readings: &[CachedData], _controller_id: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    for data in readings {
        for device in &data.devices {
            if let Some(temp) = device.value {
                if temp > 50.0 {
                    info!(
                        "Temperature above 50C for {} on {}: {}. Turn on cooling.",
                        device.id, data.client_id, temp
                    );
                    commands.push(Command::TurnOnCooling(device.id.clone()));
                } else {
                    info!(
                        "Temperature below 50C for {} on {}: {}. Cooling off.",
                        device.id, data.client_id, temp
                    );
                    commands.push(Command::TurnOffCooling(device.id.clone()));
                    // TODO: Implement energy optimization (500 kWh/30 days)
                }
            }
        }
    }
    commands
}
