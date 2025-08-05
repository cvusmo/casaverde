// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/cache.rs

use crate::models::{Command, CommandPayload, DeviceReading, SensorReading};
use log::info;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

lazy_static::lazy_static! {
    static ref TEMP_CACHE: Arc<Mutex<HashMap<String, (Vec<DeviceReading>, Instant)>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref COMMAND_CACHE: Arc<Mutex<HashMap<String, (Vec<Command>, Instant)>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Retrieve clone of current temperature cache
pub fn get_temp_cache() -> Arc<Mutex<HashMap<String, (Vec<DeviceReading>, Instant)>>> {
    TEMP_CACHE.clone()
}

/// Retrieve clone of current command cache
pub fn get_command_cache() -> Arc<Mutex<HashMap<String, (Vec<Command>, Instant)>>> {
    COMMAND_CACHE.clone()
}

/// Insert new temperature data into cache with timestamp
pub fn insert_temp_cache(client_id: String, data: SensorReading) {
    let mut cache = TEMP_CACHE.lock().unwrap();
    let timestamp = Instant::now();
    cache.insert(client_id.clone(), (data.devices, timestamp));
    info!(
        "Inserted temperature data for client {} into cache",
        client_id
    );
}

/// Insert new command data into cache with timestamp
pub fn insert_command_cache(controller_id: String, data: CommandPayload) {
    let mut cache = COMMAND_CACHE.lock().unwrap();
    let timestamp = Instant::now();
    cache.insert(controller_id.clone(), (data.commands, timestamp));
    info!(
        "Inserted command data for controller {} into cache",
        controller_id
    );
}
