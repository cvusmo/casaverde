// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/controller.rs

use log::info;
use serde_json::Value;
use tokio::time::interval;

#[derive(Debug, Clone)]
pub enum Command {
    TurnOnCooling,
    TurnOffCooling,
    TurnOnMoisture,
    TurnOffMoisture,
    OpenValve,
    CloseValve,
    TurnOnSolar,
    TurnOffSolar,
    TurnOnHumidity,
    TurnOffHumidity,
    SetPWM(u8),
    GetProbeTemp,
    TurnOnRelay2,
    TurnOffRelay2,
}

impl Command {
    pub fn id(&self) -> &str {
        match self {
            Command::TurnOnCooling | Command::TurnOffCooling => "FAN1",
            Command::TurnOnMoisture | Command::TurnOffMoisture => "moisture-1",
            Command::OpenValve | Command::CloseValve => "water-1",
            Command::TurnOnSolar | Command::TurnOffSolar => "solar-1",
            Command::TurnOnHumidity | Command::TurnOffHumidity => "humidity-1",
            Command::SetPWM(_) => "FAN1",
            Command::GetProbeTemp => "blackbeard-probe",
            Command::TurnOnRelay2 | Command::TurnOffRelay2 => "relay-2",
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
                                        device.as_object().and_then(|obj| {
                                            let id = obj.get("id")?.as_str()?;
                                            let value = obj.get("value")?.as_f64()?;
                                            match id {
                                                "blackbeard-cpu" if value > 40.0 => {
                                                    Some(Command::TurnOnCooling)
                                                }
                                                "blackbeard-probe" if value > 15.0 => {
                                                    Some(Command::TurnOnRelay2)
                                                }
                                                "blackbeard-probe" if value <= 15.0 => {
                                                    Some(Command::TurnOffRelay2)
                                                }
                                                _ => None,
                                            }
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
    let mut commands = Vec::new();
    if local_temp > 35.0 {
        commands.push(Command::TurnOnCooling);
    }
    // Periodic queries for all sensors
    commands.push(Command::GetProbeTemp);
    commands.push(Command::TurnOnMoisture);
    commands.push(Command::OpenValve);
    commands.push(Command::TurnOnSolar);
    commands.push(Command::TurnOnHumidity);
    commands
}
