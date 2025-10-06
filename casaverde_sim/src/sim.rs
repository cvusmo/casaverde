// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_sim
// sim.rs -

use std::time::{Duration, Instant};
use tokio::sync::mpsc;

/// Represents a cell in the hydroponic grid.
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub moisture: f32,     // 0.0 - 1.0
    pub nutrients: f32,    // 0.0 - 1.0
    pub plant_height: f32, // 0.0 - 1.0 normalized
}

/// Simulation configuration parameters.
#[derive(Clone, Copy, Debug)]
pub struct SimConfig {
    pub width: usize,
    pub height: usize,
    pub moisture_decay: f32,
    pub nutrient_decay: f32,
    pub growth_rate: f32,
}

/// Simulation state containing grid.
pub struct Simulation {
    pub grid: Vec<Cell>,
    pub config: SimConfig,
    last_update: Instant,
    counter: u32,
}

impl Simulation {
    /// Creates a new simulation with a grid of the specified width and height.
    pub fn new(width: usize, height: usize) -> Self {
        let config = SimConfig {
            width,
            height,
            moisture_decay: 0.01,
            nutrient_decay: 0.005,
            growth_rate: 0.002,
        };

        let grid = vec![
            Cell {
                moisture: 0.5,
                nutrients: 0.5,
                plant_height: 0.2,
            };
            width * height
        ];

        Self {
            grid,
            config,
            last_update: Instant::now(),
            counter: 0,
        }
    }

    /// Steps the simulation forward in time (fixed 100ms interval).
    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) < Duration::from_millis(100) {
            return;
        }
        self.last_update = now;
        self.counter = self.counter.wrapping_add(1);

        let time_factor = (self.counter as f32 % 3600.0) / 3600.0; // Deterministic counter-based factor

        for (i, cell) in self.grid.iter_mut().enumerate() {
            let moisture_influx = 0.01 + (i as f32 * 0.001) + time_factor * 0.01; // Deterministic based on index and counter
            let nutrient_influx = 0.005 + (i as f32 * 0.0005) + time_factor * 0.005;

            cell.moisture =
                (cell.moisture + moisture_influx - self.config.moisture_decay).clamp(0.0, 1.0);
            cell.nutrients =
                (cell.nutrients + nutrient_influx - self.config.nutrient_decay).clamp(0.0, 1.0);

            // Simplified growth model
            cell.plant_height += (cell.moisture + cell.nutrients) * self.config.growth_rate;
            cell.plant_height = cell.plant_height.clamp(0.0, 1.0);
        }
    }

    /// Retrieves a cell at the specified coordinates, if valid.
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.config.width && y < self.config.height {
            Some(&self.grid[y * self.config.width + x])
        } else {
            None
        }
    }
}

/// Runs the simulation in a background task and sends snapshots to the channel.
pub async fn run_simulation(tx: mpsc::Sender<Vec<Cell>>, width: usize, height: usize) {
    let mut sim = Simulation::new(width, height);

    loop {
        sim.update();
        if tx.send(sim.grid.clone()).await.is_err() {
            break; // Stop if channel closes
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
