// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/controller.rs

use log::info;
use serde_json::Value;
use tokio::time::interval;

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

impl Command {
    pub fn id(&self) -> &str {
        match self {
            Command::TurnOnCooling(id) => id,
            Command::TurnOffCooling(id) => id,
            Command::TurnOnMoisture(id) => id,
            Command::TurnOffMoisture(id) => id,
            Command::OpenValve(id) => id,
            Command::CloseValve(id) => id,
            Command::TurnOnSolar(id) => id,
            Command::TurnOffSolar(id) => id,
            Command::TurnOnHumidity(id) => id,
            Command::TurnOffHumidity(id) => id,
            Command::SetPWM(id, _) => id,
        }
    }
}

pub fn process_remote_readings(readings: &Value, controller_id: &str) -> Vec<Command> {
    readings
        .as_array()
        .map(|client_readings| {
            client_readings
                .iter()
                .filter_map(|client_reading| {
                    let arr: &[Value] = client_reading.as_array()?.as_slice();
                    let [id, devices] = arr else { return None };
                    (id.as_str() == Some(controller_id)).then(|| {
                        devices
                            .as_array()
                            .map(|devices_array| {
                                devices_array
                                    .iter()
                                    .filter_map(|device| {
                                        device
                                            .as_object()
                                            .and_then(|obj| {
                                                let id = obj.get("id")?.as_str()?;
                                                let value = obj.get("value")?.as_f64()?;
                                                (id == "blackbeard-cpu" && value > 40.0)
                                                    .then(|| Command::TurnOnCooling("FAN1".to_string()))
                                            })
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    })
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub fn process_local_rules(_config: &crate::config::Config, local_temp: f64) -> Vec<Command> {
    (local_temp > 35.0)
        .then(|| vec![Command::TurnOnCooling("FAN1".to_string())])
        .unwrap_or_default()
}

pub async fn run_light_timer(
    relay_id: String,
    on_hours: u64,
    off_hours: u64,
    tx: tokio::sync::mpsc::Sender<Command>,
) {
    info!("Starting light timer for relay {} with {}h ON / {}h OFF cycle", relay_id, on_hours, off_hours);
    let mut interval = interval(tokio::time::Duration::from_secs(15));
    let mut is_on = true;

    loop {
        interval.tick().await;
        let cmd = if is_on { Command::TurnOnSolar(relay_id.clone()) } else { Command::TurnOffSolar(relay_id.clone()) };
        if tx.send(cmd).await.is_err() {
            break;
        }
        info!("Toggled light to {} at {:?}", if is_on { "ON" } else { "OFF" }, tokio::time::Instant::now());
        is_on = !is_on;
    }
}
