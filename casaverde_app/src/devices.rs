use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: u64,
    pub name: String,
    pub active: bool,
    pub last_updated: u64,
}

impl Device {
    pub fn new(id: u64, name: &str, active: bool) -> Self {
        Self {
            id,
            name: name.into(),
            active,
            last_updated: now(),
        }
    }

    pub fn update_status(&mut self, active: bool) {
        self.active = active;
        self.last_updated = now();
    }
}

/// Interface trait for any backend that provides device data.
#[async_trait::async_trait]
pub trait DeviceProvider {
    async fn fetch_devices(&self) -> anyhow::Result<Vec<Device>>;
    async fn update_devices(&self, devices: Vec<Device>) -> anyhow::Result<Vec<Device>>;
    async fn toggle_device(&self, id: u64) -> anyhow::Result<()>;
}

/// Returns current UNIX timestamp.
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
