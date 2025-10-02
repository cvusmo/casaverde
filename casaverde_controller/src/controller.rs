// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/controller.rs - Data processing and command generation

use log::info;
use tokio::sync::mpsc::Sender;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct SensorState {
    pub solar: bool,
    pub temperature: bool,
    pub moisture: bool,
    pub humidity: bool,
    pub water: bool,
    pub time: bool,
}

#[derive(Debug, Clone)]
pub enum Command {
    TurnOnCooling(String),      // Temperature
    TurnOffCooling(String),     // Temperature
    TurnOnMoisture(String),     // Moisture
    TurnOffMoisture(String),    // Moisture 
    OpenValve(String),          // Water
    CloseValve(String),         // Water 
    TurnOnSolar(String),        // Solar 
    TurnOffSolar(String),       // Solar 
    TurnOnHumidity(String),     // Humidity 
    TurnOffHumidity(String),    // Humidity 
    SetPWM(String, u8),
}

pub fn process_remote_readings(readings: &Value, controller_id: &str) -> Vec<Command> {
    let mut cmds = Vec::new();
    if let Some(temp) = readings.get(controller_id).and_then(|v| v.get("temperature")) {
        if let Some(t) = temp.as_f64() {
            if t > 40.0 {
                cmds.push(Command::TurnOnCooling("FAN1".to_string()));
            }
        }
    }
    cmds
}

pub fn process_local_rules(_config: &crate::config::Config, local_temp: f64) -> Vec<Command> {
    let mut cmds = Vec::new();
    if local_temp > 35.0 {
        cmds.push(Command::TurnOnCooling("FAN1".to_string()));
    }
    cmds
}

pub async fn run_light_timer(
    light_id: String,
    on_hours: u64,
    off_hours: u64,
    tx: Sender<Command>,
) {
    loop {
        info!("Timer: turning ON {}", light_id);
        if tx.send(Command::TurnOnSolar(light_id.clone())).await.is_err() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(on_hours * 3600)).await;

        info!("Timer: turning OFF {}", light_id);
        if tx.send(Command::TurnOffSolar(light_id.clone())).await.is_err() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(off_hours * 3600)).await;
    }
}
