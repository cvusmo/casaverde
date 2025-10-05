// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceReading {
    pub id: String,
    pub value: Option<f32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub client_id: String,
    pub devices: Vec<DeviceReading>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub action: String,
    pub device_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPayload {
    pub controller_id: String,
    pub commands: Vec<Command>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigData {
    pub server: String,
    pub controller_id: String,
    pub serial_port: Option<String>,
    pub light_relay_id: String,
    pub light_on_hours: u64,
    pub light_off_hours: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEntry {
    pub current: ConfigData,
    pub backup: Option<ConfigData>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigPayload {
    pub controller_id: String,
    pub config: ConfigData,
    pub revert: bool,
}
