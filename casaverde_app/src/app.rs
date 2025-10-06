// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/app.rs

use ratatui::widgets::ListState;
use crate::devices::DeviceData;
use casaverde_sim::sim::{run_simulation, Cell};
use casaverde_utils::log::error;
use crossterm::event::{self, Event, KeyCode};
use std::io;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task;

const POLL_INTERVAL: Duration = Duration::from_millis(100);
const DEVICE_REFRESH_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Devices,
    Monitoring,
    Config,
}

pub struct App {
    pub sensor_data: DeviceData,
    pub selected: usize,
    pub simulation_rx: mpsc::Receiver<Vec<Cell>>,
    pub running: bool,
    pub screen: Screen,
    pub quit: bool,
    pub list_state: ListState, // Added list_state
}

impl App {
    pub fn new(config_path: &str, server: &str) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let sensor_data = DeviceData::new(config_path, server);
        let (tx, rx) = mpsc::channel::<Vec<Cell>>(8);

        task::spawn(run_simulation(tx, 8, 8));

        Self {
            sensor_data,
            selected: 0,
            simulation_rx: rx,
            running: true,
            screen: Screen::Devices,
            quit: false,
            list_state,
        }
    }

    pub async fn update(&mut self) {
        if let Err(e) = self.sensor_data.update_devices().await {
            error!("Device update failed: {:?}", e);
        }

        if let Ok(Some(sim_data)) =
            tokio::time::timeout(Duration::from_millis(10), self.simulation_rx.recv()).await
        {
            let avg_height: f32 =
                sim_data.iter().map(|c| c.plant_height).sum::<f32>() / sim_data.len() as f32;

            if !self.sensor_data.devices.is_empty() {
                self.sensor_data.devices[0].value = Some(avg_height * 100.0);
            }
        }
    }

    pub fn toggle_selected(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.sensor_data.devices.len() {
                self.sensor_data.toggle_sensor(selected);
            }
        }
    }

    pub fn move_up(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let new_selected = if selected > 0 {
                selected - 1
            } else {
                self.sensor_data.devices.len().saturating_sub(1)
            };
            self.list_state.select(Some(new_selected));
        }
    }

    pub fn move_down(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let new_selected = (selected + 1) % self.sensor_data.devices.len();
            self.list_state.select(Some(new_selected));
        }
    }

    pub fn switch_screen(&mut self) {
        self.screen = match self.screen {
            Screen::Devices => Screen::Monitoring,
            Screen::Monitoring => Screen::Config,
            Screen::Config => Screen::Devices,
        };
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}

pub async fn run_app(tui: &mut crate::tui::Tui, app: &mut App) -> io::Result<()> {
    let mut last_refresh = Instant::now();

    loop {
        tui.draw(app)?;

        if event::poll(POLL_INTERVAL)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => app.move_up(),
                    KeyCode::Down => app.move_down(),
                    KeyCode::Enter => app.toggle_selected(),
                    KeyCode::Char('r') => {
                        app.sensor_data.update_devices().await?;
                    }
                    KeyCode::Char('t') => {
                        app.toggle_selected();
                    }
                    KeyCode::Char('m') | KeyCode::Char('c') | KeyCode::Char('s') => {
                        app.switch_screen();
                    }
                    _ => {}
                },
                _ => {},
            }
        }

        if last_refresh.elapsed() >= DEVICE_REFRESH_INTERVAL {
            app.update().await;
            last_refresh = Instant::now();
        }

        if app.quit {
            break;
        }
    }

    Ok(())
}
