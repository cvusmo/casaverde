// Copyright 2025 Acris Software Ltd. Co. All Rights Reserved.
// github.com/cvusmo/casaverde/casaverde_app
// src/tui.rs

use crate::app::{App, Screen};
use crate::devices::DeviceData;
use crate::models::ConfigEntry;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    Frame, Terminal,
};
use std::io::{self, Stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        Ok(Self {
            terminal: Terminal::new(backend)?,
        })
    }

    pub fn enter(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    pub fn exit(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut crate::app::App) -> io::Result<()> {
        self.terminal.draw(|f| render_ui(f, app))?;
        Ok(())
    }
}

fn render_ui(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(80),
            Constraint::Length(3),
        ])
        .split(area);

    let title = Paragraph::new("Casaverde")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    match app.screen {
        Screen::Devices => render_devices(f, chunks[1], app),
        Screen::Monitoring => render_monitoring(f, chunks[1], app),
        Screen::Config => render_config(f, chunks[1], app),
    }

    let status = Paragraph::new("q:quit ↑/↓:navigate Enter:toggle m/c/s:switch")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);
    f.render_widget(status, chunks[2]);
}

fn render_devices(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = app
        .sensor_data
        .devices
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let status = if d.active {
                Span::styled("ON", Style::default().fg(Color::Green))
            } else {
                Span::styled("OFF", Style::default().fg(Color::Red))
            };
            let relay = app.sensor_data.config.iter().any(|c| {
                c.current.controller_id == d.id
                    && c.current
                        .serial_port
                        .as_ref()
                        .map_or(false, |p| p.contains("relay"))
            });
            let label = if relay {
                format!("Relay {} [{}] (Ctrl: {})", i + 1, status.content, d.id)
            } else {
                format!("Sensor {} [{}] ({})", i + 1, status.content, d.id)
            };
            ListItem::new(Line::from(Span::raw(label)))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Devices").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_monitoring(f: &mut Frame, area: Rect, app: &App) {
    let rows: Vec<Row> = app
        .sensor_data
        .devices
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let sensor_name = app
                .sensor_data
                .config
                .iter()
                .find(|c| c.current.controller_id == d.id)
                .map_or(d.id.clone(), |c| c.current.controller_id.clone());
            let sensor_type = app
                .sensor_data
                .config
                .iter()
                .find(|c| c.current.controller_id == d.id)
                .and_then(|c| c.current.serial_port.clone())
                .map_or("unknown".to_string(), |p| {
                    if p.contains("temperature") {
                        "temperature".to_string()
                    } else if p.contains("moisture") {
                        "moisture".to_string()
                    } else if p.contains("nutrients") {
                        "nutrients".to_string()
                    } else if p.contains("humidity") {
                        "humidity".to_string()
                    } else if p.contains("solar") {
                        "solar".to_string()
                    } else if p.contains("water") {
                        "water".to_string()
                    } else if p.contains("relay") {
                        "relay".to_string()
                    } else {
                        "unknown".to_string()
                    }
                });
            let sensor_num = (i + 1).to_string();
            let reading = format!("{:.1}", d.value.unwrap_or_default());
            Row::new(vec![
                Cell::from(Span::raw(sensor_name)),
                Cell::from(Span::raw(sensor_type)),
                Cell::from(Span::raw(sensor_num)),
                Cell::from(Span::raw(reading)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        vec![
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ],
    )
    .block(Block::default().title("Monitoring").borders(Borders::ALL))
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Sensor Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Sensor Type",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Sensor #",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Reading",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]));
    f.render_widget(table, area);
}

fn render_config(f: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = vec![
        Line::from(Span::raw("# Configuration")),
        Line::from(Span::raw("| Sensor ID | Type | Endpoint | Interval (s) | Serial Port | Light Relay ID | Light On (h) | Light Off (h) |")),
        Line::from(Span::raw("|-----------|------|----------|--------------|-------------|-----------------|--------------|---------------|")),
    ];
    for cfg in &app.sensor_data.config {
        let line = format!(
            "| {} | {} | N/A | N/A | {} | {} | {} | {} |",
            cfg.current.controller_id,
            cfg.current.serial_port.as_deref().unwrap_or("N/A"),
            cfg.current.serial_port.as_deref().unwrap_or("N/A"),
            cfg.current.light_relay_id,
            cfg.current.light_on_hours,
            cfg.current.light_off_hours
        );
        lines.push(Line::from(Span::raw(line)));
    }

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Config").borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Handles user input for the TUI.
pub fn handle_input(app: &mut App) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(10))? {
        if let Event::Key(k) = event::read()? {
            match k.code {
                KeyCode::Char('q') => app.quit = true,
                KeyCode::Up => app.move_up(),
                KeyCode::Down => app.move_down(),
                KeyCode::Enter => {
                    if let Some(selected) = app.list_state.selected() {
                        if selected < app.sensor_data.devices.len() {
                            app.sensor_data.toggle_sensor(selected);
                            let device = &app.sensor_data.devices[selected];
                            if let Some(cfg) = app.sensor_data.config.get(selected) {
                                let cmd = if device.active {
                                    format!("SET {} ON", device.id)
                                } else {
                                    format!("SET {} OFF", device.id)
                                };
                                println!("Sending command to controller: {}", cmd);
                            }
                        }
                    }
                }
                KeyCode::Char('m') => app.screen = Screen::Monitoring,
                KeyCode::Char('c') => app.screen = Screen::Config,
                KeyCode::Char('d') => app.screen = Screen::Devices,
                _ => {}
            }
        }
    }
    Ok(())
}
