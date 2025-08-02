// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/handlers.rs

use axum::{response::Json, extract::Json as AxumJson, http::StatusCode};
use crate::models::{TempData, SensorReading};
use crate::cache::{get_cache, insert_cache};

pub async fn get_temperatures() -> Result<Json<Vec<(String, TempData)>>, (StatusCode, String)> {
    let cache = get_cache();
    let cache_data = cache.lock().unwrap();
    if cache_data.is_empty() {
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
    eprintln!("Received POST data: client_id={}, temp_data={:?}", data.client_id, data.temp_data);
    insert_cache(data.client_id.clone(), (data.temp_data.clone(), std::time::Instant::now()));
}

pub async fn get_all_data() -> Json<Vec<(String, TempData)>> {
    let cache = get_cache();
    Json(cache.lock().unwrap().iter().map(|(id, (data, _))| (id.clone(), data.clone())).collect())
}
