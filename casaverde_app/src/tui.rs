// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/tui.rs

use crate::{
    app::{CasaverdeApp, Screen},
    devices::Sensor,
    ui::create_layout,
};
use crossterm::event::{self, Event, KeyCode};
use log::info;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io;

pub fn render_tui(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &CasaverdeApp,
) -> io::Result<()> {
    terminal.draw(|frame| {
        let chunks = create_layout(frame.area());

        let title = Paragraph::new("Casaverde")
            .block(Block::new().borders(Borders::ALL))
            .style(Style::default().fg(Color::Green))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(title, chunks[0]);

        match app.screen {
            Screen::Devices => {
                let mut items = Vec::with_capacity(app.sensor_data.active_count);
                for i in 0..app.sensor_data.active_count {
                    let id = app.sensor_data.config.configs[i]
                        .id
                        .trim_matches(char::from(0));
                    let value = app.sensor_data.device_values[i];
                    let sensor = match id {
                        "blackbeard-cpu" => Some(Sensor::Temperature),
                        "solar-1" => Some(Sensor::Solar),
                        "moisture-1" => Some(Sensor::Moisture),
                        "humidity-1" => Some(Sensor::Humidity),
                        "water-1" => Some(Sensor::Water),
                        "blackbeard-probe" => Some(Sensor::Probe),
                        _ => None,
                    };

                    let flag = if sensor.map_or(false, |s| app.sensor_data.states[s as usize]) {
                        "[ON]  "
                    } else {
                        "[OFF] "
                    };

                    let value_str = match (sensor, value) {
                        (Some(Sensor::Temperature), Some(v)) => format!("{v:.1}°C"),
                        (Some(Sensor::Solar), Some(v)) => format!("{v:.1}W"),
                        (Some(Sensor::Moisture), Some(v)) => format!("{v:.1}%"),
                        (Some(Sensor::Humidity), Some(v)) => format!("{v:.1}%"),
                        (Some(Sensor::Water), Some(v)) => format!("{v:.1}%"),
                        (Some(Sensor::Probe), Some(v)) => format!("{v:.1}°C"),
                        (Some(_), None) => "N/A".to_string(),
                        (None, Some(v)) if id == "relay-1" => format!("{v:.1}"),
                        _ => "N/A".to_string(),
                    };

                    items.push(ListItem::new(Span::raw(format!(
                        "{flag} {}: {}",
                        sensor.map_or(id, |s| s.name()),
                        value_str
                    ))));
                    info!("Rendering device {i}: id={id}, value={value:?}");
                }

                let mut list_state = ListState::default();
                list_state.select(Some(app.selected.min(items.len().saturating_sub(1))));

                let devices = List::new(items.clone())
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Devices")
                            .title_alignment(ratatui::layout::Alignment::Center)
                            .style(Style::default().fg(Color::Yellow)),
                    )
                    .highlight_symbol(">> ")
                    .highlight_style(Style::default().bg(Color::DarkGray));

                frame.render_stateful_widget(devices, chunks[1], &mut list_state);
                info!("Rendered Devices screen with {} items", items.len());
            }
            Screen::Monitoring => {
                let mut temp_text = Vec::with_capacity(app.sensor_data.active_count);
                for i in 0..app.sensor_data.active_count {
                    let id = app.sensor_data.config.configs[i]
                        .id
                        .trim_matches(char::from(0));
                    let value = app.sensor_data.device_values[i];
                    let sensor = match id {
                        "blackbeard-cpu" => Some(Sensor::Temperature),
                        "solar-1" => Some(Sensor::Solar),
                        "moisture-1" => Some(Sensor::Moisture),
                        "humidity-1" => Some(Sensor::Humidity),
                        "water-1" => Some(Sensor::Water),
                        "blackbeard-probe" => Some(Sensor::Probe),
                        _ => None,
                    };

                    let value_str = match (sensor, value) {
                        (Some(Sensor::Temperature), Some(v)) => format!("{v:.1}°C"),
                        (Some(Sensor::Solar), Some(v)) => format!("{v:.1}W"),
                        (Some(Sensor::Moisture), Some(v)) => format!("{v:.1}%"),
                        (Some(Sensor::Humidity), Some(v)) => format!("{v:.1}%"),
                        (Some(Sensor::Water), Some(v)) => format!("{v:.1}%"),
                        (Some(Sensor::Probe), Some(v)) => format!("{v:.1}°C"),
                        (Some(_), None) => "N/A".to_string(),
                        (None, Some(v)) if id == "relay-1" => format!("{v:.1}"),
                        _ => "N/A".to_string(),
                    };

                    temp_text.push(
                        Line::from(format!(
                            "{}: {}",
                            sensor.map_or(id, |s| s.name()),
                            value_str
                        ))
                        .centered(),
                    );
                    info!("Monitoring device {i}: id={id}, value={value:?}");
                }

                if temp_text.is_empty() {
                    temp_text.push(Line::from("No devices configured").centered());
                }

                let monitor = Paragraph::new(temp_text.clone())
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Monitoring")
                            .title_alignment(ratatui::layout::Alignment::Center)
                            .style(Style::default().fg(Color::Yellow)),
                    )
                    .alignment(ratatui::layout::Alignment::Center);

                frame.render_widget(monitor, chunks[1]);
                info!("Rendered Monitoring screen with {} items", temp_text.len());
            }
            Screen::Config => {
                let mut config_text = Vec::new();
                config_text.push(Line::from("Configuration").centered());
                for config in &app.sensor_data.config.configs {
                    config_text.push(
                        Line::from(format!(
                            "{}: type={}, endpoint={}, interval={}s, serial_port={}",
                            config.id,
                            config.r#type,
                            config.endpoint,
                            config.interval,
                            config.serial_port
                        ))
                        .centered(),
                    );
                }
                config_text.push(
                    Line::from(format!("Server: {}", app.sensor_data.config.server)).centered(),
                );

                let config = Paragraph::new(config_text.clone())
                    .block(
                        Block::new()
                            .borders(Borders::ALL)
                            .title("Configuration")
                            .title_alignment(ratatui::layout::Alignment::Center)
                            .style(Style::default().fg(Color::Yellow)),
                    )
                    .alignment(ratatui::layout::Alignment::Center);

                frame.render_widget(config, chunks[1]);
                info!("Rendered Config screen with {} items", config_text.len());
            }
        }

        let status_text = match app.screen {
            Screen::Devices => {
                "Navigate with Up/Down, Toggle with Enter, Switch with m/c, Quit with q"
            }
            Screen::Monitoring => "Switch to Devices with s, Config with c, Quit with q",
            Screen::Config => "Switch to Devices with s, Monitoring with m, Quit with q",
        };
        let status = Paragraph::new(status_text)
            .block(Block::new().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(status, chunks[2]);
    })?;
    Ok(())
}

pub fn handle_tui_events(app: &mut CasaverdeApp) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(200))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    app.quit();
                    info!("Quit command received");
                }
                KeyCode::Up if app.screen == Screen::Devices => app.move_up(),
                KeyCode::Down if app.screen == Screen::Devices => app.move_down(),
                KeyCode::Char('m') => {
                    app.switch_screen();
                    info!("Switched to Monitoring screen");
                }
                KeyCode::Char('c') => {
                    app.switch_screen();
                    info!("Switched to Config screen");
                }
                KeyCode::Char('s') => {
                    app.switch_screen();
                    info!("Switched to Devices screen");
                }
                KeyCode::Enter => {
                    app.toggle_selected_sensor();
                    info!("Toggled selected sensor with Enter");
                }
                _ => {}
            }
        }
    }
    Ok(())
}
