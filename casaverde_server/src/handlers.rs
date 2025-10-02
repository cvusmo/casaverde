// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/handlers.rs

use axum::{response::Json, extract::Json as AxumJson, extract::Path, http::StatusCode};
use crate::models::{ConfigEntry, ConfigPayload, DeviceReading, SensorReading, Command, CommandPayload};
use crate::cache::{get_config_cache, insert_config_cache, get_temp_cache, insert_temp_cache, get_command_cache, insert_command_cache};
use log::{info, error};

pub async fn get_temperatures() -> Result<Json<Vec<(String, Vec<DeviceReading>)>>, (StatusCode, String)> {
    let cache = get_temp_cache();
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

pub async fn get_all_data() -> Json<Vec<(String, Vec<DeviceReading>)>> {
    let cache = get_temp_cache();
    Json(cache.lock().unwrap().iter().map(|(id, (data, _))| (id.clone(), data.clone())).collect())
}

pub async fn get_commands() -> Result<Json<Vec<(String, Vec<Command>)>>, (StatusCode, String)> {
    let cache = get_command_cache();
    let cache_data = cache.lock().unwrap();
    if cache_data.is_empty() {
        error!("No command data available");
        Err((StatusCode::NOT_FOUND, "No command data available".to_string()))
    } else {
        Ok(Json(
            cache_data
                .iter()
                .map(|(id, (commands, _))| (id.clone(), commands.clone()))
                .collect()
        ))
    }
}

pub async fn get_configs(Path(controller_id): Path<String>) -> Result<Json<ConfigEntry>, (StatusCode, String)> {
    let cache = get_config_cache();
    let cache_data = cache.lock().unwrap();
    cache_data.get(&controller_id)
        .cloned()
        .map(Json)
        .ok_or((StatusCode::NOT_FOUND, "Config not found".to_string()))
}


pub async fn post_commands(AxumJson(data): AxumJson<CommandPayload>) {
    info!("Received POST commands: controller_id={}, commands={:?}", data.controller_id, data.commands);
    insert_command_cache(data.controller_id.clone(), data);
}

pub async fn post_configs(AxumJson(payload): AxumJson<ConfigPayload>) {
    insert_config_cache(payload.controller_id.clone(), payload.config, payload.revert);
}

pub async fn post_sensor_data(AxumJson(data): AxumJson<SensorReading>) {
    info!("Received POST data: client_id={}, devices={:?}", data.client_id, data.devices);
    insert_temp_cache(data.client_id.clone(), data);
}
