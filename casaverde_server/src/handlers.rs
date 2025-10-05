// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/handlers.rs

use axum::{response::Json, extract::Json as AxumJson, extract::Path};
use crate::models::{SensorReading, Command, CommandPayload, ConfigPayload, ConfigData, ConfigEntry, DeviceReading};
use crate::cache::{temp_cache, command_cache, config_cache, insert_temp, insert_command, insert_config};

pub async fn get_temperatures() -> Json<Vec<(String, Vec<DeviceReading>)>> {
    let cache_ref = temp_cache();
    let cache = cache_ref.read().await;
    let result = cache.iter()
        .map(|(id, (data, _))| (id.clone(), data.clone()))
        .collect();
    Json(result)
}

pub async fn get_all_data() -> Json<Vec<(String, Vec<DeviceReading>)>> {
    get_temperatures().await
}

pub async fn get_commands() -> Json<Vec<(String, Vec<Command>)>> {
    let cache_ref = command_cache();
    let cache = cache_ref.read().await;
    let result = cache.iter()
        .map(|(id, (cmds, _))| (id.clone(), cmds.clone()))
        .collect();
    Json(result)
}

pub async fn get_configs(Path(controller_id): Path<String>) -> Json<ConfigEntry> {
    let cache_ref = config_cache();
    let cache = cache_ref.read().await;
    let entry = cache.get(&controller_id).cloned().unwrap_or(ConfigEntry {
        current: ConfigData {
            server: "".to_string(),
            controller_id: "".to_string(),
            serial_port: None,
            light_relay_id: "".to_string(),
            light_on_hours: 0,
            light_off_hours: 0,
        },
        backup: None,
    });
    Json(entry)
}

pub async fn post_sensor_data(AxumJson(data): AxumJson<SensorReading>) {
    let client_id = data.client_id.clone(); 
    if !client_id.is_empty() {
        insert_temp(client_id, data).await;
    }
}

pub async fn post_commands(AxumJson(payload): AxumJson<CommandPayload>) {
    let controller_id = payload.controller_id.clone();
    if !controller_id.is_empty() {
        insert_command(controller_id, payload.commands).await;
    }
}

pub async fn post_configs(AxumJson(payload): AxumJson<ConfigPayload>) {
    let controller_id = payload.controller_id.clone();
    if !controller_id.is_empty() {
        insert_config(controller_id, payload.config, payload.revert).await;
    }
}

