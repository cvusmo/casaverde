// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/cache.rs

use crate::models::{DeviceReading, SensorReading};
use log::info;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

lazy_static::lazy_static! {
    static ref CACHE: Arc<Mutex<HashMap<String, (Vec<DeviceReading>, Instant)>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Retrieve clone of current cache
pub fn get_cache() -> Arc<Mutex<HashMap<String, (Vec<DeviceReading>, Instant)>>> {
    CACHE.clone()
}

/// Insert new data into cache with timestamp
pub fn insert_cache(client_id: String, data: SensorReading) {
    let mut cache = CACHE.lock().unwrap();
    let timestamp = Instant::now();
    cache.insert(client_id.clone(), (data.devices, timestamp));
    info!("Inserted data for client {} into cache", client_id);
}
