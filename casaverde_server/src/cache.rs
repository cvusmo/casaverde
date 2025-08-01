// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_server
// src/cache.rs

use crate::models::TempData;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

lazy_static::lazy_static! {
    static ref CACHE: Arc<Mutex<HashMap<String, (TempData, Instant)>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Retrieve clone of current cache
pub fn get_cache() -> Arc<Mutex<HashMap<String, (TempData, Instant)>>> {
    CACHE.clone()
}

/// Insert new data into cache with timestamp
pub fn insert_cache(client_id: String, data: (TempData, Instant)) {
    let mut cache = CACHE.lock().unwrap();
    cache.insert(client_id, data);
}
