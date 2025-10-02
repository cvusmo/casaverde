// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_controller
// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigEntry {
    pub current: ConfigData,
    pub backup: Option<ConfigData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigData {
    pub server: String,
    pub controller_id: String,
    pub serial_port: Option<String>,
    pub light_relay_id: String,
    pub light_on_hours: u64,
    pub light_off_hours: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceReading {
    pub id: String,
    pub value: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SensorReading {
    pub client_id: String,
    pub devices: Vec<DeviceReading>,
}
