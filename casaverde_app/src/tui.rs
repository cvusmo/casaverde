// Copyright 2025 Nicholas Jordan. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/tui.rs

// Purpose:
// Handles TUI rendering and key event processing

use crate::{app::CasaverdeApp, sensors::Sensor, ui::create_layout};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::io;

pub fn render_tui(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &CasaverdeApp,
) -> io::Result<()> {
    terminal.draw(|frame| {
        let chunks = create_layout(frame.area());

        // Title
        let title = Paragraph::new("Casaverde")
            .block(Block::new().borders(Borders::ALL))
            .style(Style::default().fg(Color::Green))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(title, chunks[0]);

        // Sensor list
        let items: Vec<ListItem> = Sensor::ALL
            .iter()
            .enumerate()
            .map(|(i, &sensor)| {
                let flag = if app.sensor_data.states[i] {
                    "[ON]  "
                } else {
                    "[OFF] "
                };
                let value = if sensor == Sensor::Temperature && app.sensor_data.states[i] {
                    match (app.sensor_data.temp_data.cpu, app.sensor_data.temp_data.gpu) {
                        (Some(cpu), Some(gpu)) => {
                            format!(" (CPU: {:.1}°C, GPU: {:.1}°C)", cpu, gpu)
                        }
                        (Some(cpu), None) => format!(" (CPU: {:.1}°C, GPU: N/A)", cpu),
                        (None, Some(gpu)) => format!(" (CPU: N/A, GPU: {:.1}°C)", gpu),
                        (None, None) => " (CPU: N/A, GPU: N/A)".to_string(),
                    }
                } else {
                    String::new()
                };
                ListItem::new(Span::raw(format!("{}{}{}", flag, sensor.name(), value)))
            })
            .collect();

        let mut list_state = ListState::default();
        list_state.select(Some(app.selected));

        let sensors = List::new(items)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Sensors")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .style(Style::default().fg(Color::Yellow)),
            )
            .highlight_symbol(">> ")
            .highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_stateful_widget(sensors, chunks[1], &mut list_state);

        // Status
        let status_text = if app.sensor_data.states[app.selected] {
            match Sensor::ALL[app.selected] {
                Sensor::Temperature => {
                    match (app.sensor_data.temp_data.cpu, app.sensor_data.temp_data.gpu) {
                        (Some(cpu), Some(gpu)) => format!(
                            "Toggle with Enter, Quit with q (CPU: {:.1}°C, GPU: {:.1}°C)",
                            cpu, gpu
                        ),
                        (Some(cpu), None) => format!(
                            "Toggle with Enter, Quit with q (CPU: {:.1}°C, GPU: N/A)",
                            cpu
                        ),
                        (None, Some(gpu)) => format!(
                            "Toggle with Enter, Quit with q (CPU: N/A, GPU: {:.1}°C)",
                            gpu
                        ),
                        (None, None) => {
                            "Toggle with Enter, Quit with q (CPU: N/A, GPU: N/A)".to_string()
                        }
                    }
                }
                _ => format!(
                    "Toggle with Enter, Quit with q ({})",
                    Sensor::ALL[app.selected].name()
                ),
            }
        } else {
            format!(
                "Toggle with Enter, Quit with q ({})",
                Sensor::ALL[app.selected].name()
            )
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
                KeyCode::Char('q') => app.quit(),
                KeyCode::Up => app.move_up(),
                KeyCode::Down => app.move_down(),
                KeyCode::Enter => app.sensor_data.toggle_sensor(app.selected),
                _ => {}
            }
        }
    }
    Ok(())
}
