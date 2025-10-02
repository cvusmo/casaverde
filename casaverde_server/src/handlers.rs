// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/handlers.rs

use crate::models::ConfigData;
use axum::{response::Json, extract::Json as AxumJson, extract::Path};
use crate::models::{ConfigEntry, ConfigPayload, DeviceReading, SensorReading, Command, CommandPayload};
use crate::cache::{get_config_cache, insert_config_cache, get_temp_cache, insert_temp_cache, get_command_cache, insert_command_cache};
use log::info;

pub async fn get_temperatures() -> Json<Vec<(String, Vec<DeviceReading>)>> {
    let cache = get_temp_cache();
    let cache_data = cache.lock().unwrap();
    Json(
        cache_data
            .iter()
            .map(|(id, (data, _))| (id.clone(), data.clone()))
            .collect()
    )
}

pub async fn get_all_data() -> Json<Vec<(String, Vec<DeviceReading>)>> {
    let cache = get_temp_cache();
    let cache_data = cache.lock().unwrap();
    Json(cache_data.iter().map(|(id, (data, _))| (id.clone(), data.clone())).collect())
}

pub async fn get_commands() -> Json<Vec<(String, Vec<Command>)>> {
    let cache = get_command_cache();
    let cache_data = cache.lock().unwrap();
    Json(cache_data.iter().map(|(id, (commands, _))| (id.clone(), commands.clone())).collect())
}

pub async fn get_configs(Path(controller_id): Path<String>) -> Json<ConfigEntry> {
    let cache = get_config_cache();
    let cache_data = cache.lock().unwrap();
    Json(cache_data.get(&controller_id).cloned().unwrap_or(ConfigEntry {
        current: ConfigData {
            server: "".to_string(),
            controller_id: "".to_string(),
            serial_port: None,
            light_relay_id: "".to_string(),
            light_on_hours: 0,
            light_off_hours: 0,
        },
        backup: None,
    }))
}


pub async fn post_commands(AxumJson(data): AxumJson<CommandPayload>) {
    info!("Received POST commands: controller_id={}, commands={:?}", data.controller_id, data.commands);
    insert_command_cache(data.controller_id.clone(), data.commands);
}

pub async fn post_configs(AxumJson(payload): AxumJson<ConfigPayload>) {
    insert_config_cache(payload.controller_id.clone(), payload.config, payload.revert);
}

pub async fn post_sensor_data(AxumJson(data): AxumJson<SensorReading>) {
    info!("Received POST data: client_id={}, devices={:?}", data.client_id, data.devices);
    insert_temp_cache(data.client_id.clone(), data);
}
