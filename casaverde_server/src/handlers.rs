// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/handlers.rs

use axum::{response::Json, extract::Json as AxumJson, http::StatusCode};
use crate::models::{DeviceReading, SensorReading};
use crate::cache::{get_cache, insert_cache};
use log::{info, error};

pub async fn get_temperatures() -> Result<Json<Vec<(String, Vec<DeviceReading>)>>, (StatusCode, String)> {
    let cache = get_cache();
    let cache_data = cache.lock().unwrap();
    if cache_data.is_empty() {
        error!("No client temperature data available");
        Err((StatusCode::NOT_FOUND, "No client temperature data available".to_string()))
    } else {
        Ok(Json(
            cache_data
                .iter()
                .map(|(id, (data, _))| (id.clone(), data.clone()))
                .collect()
        ))
    }
}

pub async fn post_sensor_data(AxumJson(data): AxumJson<SensorReading>) {
    info!("Received POST data: client_id={}, devices={:?}", data.client_id, data.devices);
    insert_cache(data.client_id.clone(), data);
}

pub async fn get_all_data() -> Json<Vec<(String, Vec<DeviceReading>)>> {
    let cache = get_cache();
    Json(cache.lock().unwrap().iter().map(|(id, (data, _))| (id.clone(), data.clone())).collect())
}
