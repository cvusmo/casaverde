// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/main.rs

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use casaverde_server::handlers;

#[tokio::main]
async fn main() {
    let addr = get_server_addr();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server running on {addr}");

    let app = Router::new()
        .route("/temps", get(handlers::get_temperatures))
        .route("/sensor_data", get(handlers::get_all_data).post(handlers::post_sensor_data));

    axum::serve(listener, app).await.unwrap();
}

fn get_server_addr() -> SocketAddr {
    let uuid = std::fs::read_to_string("/sys/class/dmi/id/product_uuid")
        .unwrap_or_else(|_| "default-uuid".to_string())
        .trim()
        .to_string();
    let ip = format!("10.0.0.{}", uuid.split('-').next().unwrap().parse::<u8>().unwrap() % 255);
    format!("{}:3000", ip).parse().unwrap()
}
