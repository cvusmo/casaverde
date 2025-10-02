// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/controller.rs

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
    TurnOnCooling(String),
    TurnOffCooling(String),
    TurnOnMoisture(String),
    TurnOffMoisture(String),
    OpenValve(String),
    CloseValve(String),
    TurnOnSolar(String),
    TurnOffSolar(String),
    TurnOnHumidity(String),
    TurnOffHumidity(String),
    SetPWM(String, u8),
}

pub fn process_remote_readings(readings: &Value, controller_id: &str) -> Vec<Command> {
    let mut cmds = Vec::new();
    if let Some(client_readings) = readings.as_array() {
        for client_reading in client_readings.iter() {
            if let Some(arr) = client_reading.as_array() {
                if let (Some(id), Some(devices)) = (arr.get(0).and_then(|id| id.as_str()), arr.get(1)) {
                    if id == controller_id {
                        if let Some(devices_array) = devices.as_array() {
                            for device in devices_array {
                                if let Some(obj) = device.as_object() {
                                    let id = obj.get("id").and_then(|v| v.as_str()).unwrap_or("");
                                    let value = obj.get("value").and_then(|v| v.as_f64());
                                    if id == "blackbeard-cpu" && value.unwrap_or(0.0) > 40.0 {
                                        cmds.push(Command::TurnOnCooling("FAN1".to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
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
