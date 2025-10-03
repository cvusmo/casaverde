// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/cache.rs

use crate::models::{Command, ConfigData, ConfigEntry, DeviceReading, SensorReading};
use log::info;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::Instant;

type TempCacheValue = (Vec<DeviceReading>, Instant);
type CommandCacheValue = (Vec<Command>, Instant);
type ConfigCacheValue = ConfigEntry;

lazy_static::lazy_static! {
    static ref TEMP_CACHE: Arc<Mutex<HashMap<String, TempCacheValue>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref COMMAND_CACHE: Arc<Mutex<HashMap<String, CommandCacheValue>>> = Arc::new(Mutex::new(HashMap::new()));
    static ref CONFIG_CACHE: Arc<Mutex<HashMap<String, ConfigCacheValue>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn get_temp_cache() -> Arc<Mutex<HashMap<String, TempCacheValue>>> {
    TEMP_CACHE.clone()
}

pub fn get_command_cache() -> Arc<Mutex<HashMap<String, CommandCacheValue>>> {
    COMMAND_CACHE.clone()
}

pub fn get_config_cache() -> Arc<Mutex<HashMap<String, ConfigCacheValue>>> {
    CONFIG_CACHE.clone()
}

pub fn insert_temp_cache(client_id: String, data: SensorReading) {
    let mut cache = TEMP_CACHE.lock().unwrap();
    let timestamp = Instant::now();
    cache.insert(client_id.clone(), (data.devices, timestamp));
    info!(
        "Inserted temperature data for client {} into cache",
        client_id
    );
}

pub fn insert_command_cache(controller_id: String, commands: Vec<Command>) {
    let mut cache = COMMAND_CACHE.lock().unwrap();
    let timestamp = Instant::now();
    cache.insert(controller_id.clone(), (commands, timestamp));
    info!(
        "Inserted command data for controller {} into cache",
        controller_id
    );
}

pub fn insert_config_cache(controller_id: String, new_config: ConfigData, revert: bool) {
    let mut cache = CONFIG_CACHE.lock().unwrap();
    if revert {
        if let Some(entry) = cache.get_mut(&controller_id) {
            if let Some(backup) = entry.backup.take() {
                entry.backup = Some(entry.current.clone());
                entry.current = backup;
                info!("Reverted config for {}", controller_id);
            }
        }
    } else {
        let entry = cache
            .entry(controller_id.clone())
            .or_insert_with(|| ConfigEntry {
                current: new_config.clone(),
                backup: None,
            });
        entry.backup.get_or_insert(entry.current.clone());
        entry.current = new_config;
        info!("Updated config for {}", controller_id);
    }
}
