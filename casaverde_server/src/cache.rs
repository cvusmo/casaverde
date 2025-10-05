// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/cache.rs

use crate::models::{Command, ConfigData, ConfigEntry, DeviceReading, SensorReading};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Instant, Duration};

type TempCacheValue = (Vec<DeviceReading>, Instant);
type CommandCacheValue = (Vec<Command>, Instant);

lazy_static::lazy_static! {
    static ref TEMP_CACHE: Arc<RwLock<HashMap<String, TempCacheValue>>> = Arc::new(RwLock::new(HashMap::new()));
    static ref COMMAND_CACHE: Arc<RwLock<HashMap<String, CommandCacheValue>>> = Arc::new(RwLock::new(HashMap::new()));
    static ref CONFIG_CACHE: Arc<RwLock<HashMap<String, ConfigEntry>>> = Arc::new(RwLock::new(HashMap::new()));
}

pub fn temp_cache() -> Arc<RwLock<HashMap<String, TempCacheValue>>> { TEMP_CACHE.clone() }
pub fn command_cache() -> Arc<RwLock<HashMap<String, CommandCacheValue>>> { COMMAND_CACHE.clone() }
pub fn config_cache() -> Arc<RwLock<HashMap<String, ConfigEntry>>> { CONFIG_CACHE.clone() }

pub async fn insert_temp(client_id: String, data: SensorReading) {
    let mut cache = TEMP_CACHE.write().await;
    let timestamp = Instant::now();
    cache.entry(client_id.clone())
        .and_modify(|(existing, _)| {
            for d in &data.devices {
                match existing.iter_mut().find(|e| e.id == d.id) {
                    Some(e) => *e = d.clone(),
                    None => existing.push(d.clone()),
                }
            }
        })
        .or_insert((data.devices, timestamp));
}

pub async fn insert_command(controller_id: String, commands: Vec<Command>) {
    let mut cache = COMMAND_CACHE.write().await;
    cache.insert(controller_id, (commands, Instant::now()));
}

pub async fn insert_config(controller_id: String, config: ConfigData, revert: bool) {
    let mut cache = CONFIG_CACHE.write().await;
    if revert {
        if let Some(entry) = cache.get_mut(&controller_id) {
            if let Some(backup) = entry.backup.take() {
                entry.backup = Some(entry.current.clone());
                entry.current = backup;
            }
        }
    } else {
        let entry = cache.entry(controller_id).or_insert_with(|| ConfigEntry {
            current: config.clone(),
            backup: None,
        });
        entry.backup.get_or_insert(entry.current.clone());
        entry.current = config;
    }
}

pub async fn clean_temp(ttl: Duration) {
    let mut cache = TEMP_CACHE.write().await;
    let now = Instant::now();
    cache.retain(|_, (_, ts)| now.duration_since(*ts) < ttl);
}
