// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/models.rs

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub action: String,
    pub device_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandPayload {
    pub controller_id: String,
    pub commands: Vec<Command>,
}
