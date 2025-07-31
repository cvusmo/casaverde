use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{
    io::{self, Stdout},
    time::Duration,
};

/// Your five sensors
#[derive(Copy, Clone)]
enum Sensor {
    Solar,
    Temperature,
    Moisture,
    Humidity,
    Sunlight,
}

impl Sensor {
    /// All sensors in display order
    const ALL: [Sensor; 5] = [
        Sensor::Solar,
        Sensor::Temperature,
        Sensor::Moisture,
        Sensor::Humidity,
        Sensor::Sunlight,
    ];

    /// Human‐readable name
    fn name(self) -> &'static str {
        match self {
            Sensor::Solar => "Solar Sensor",
            Sensor::Temperature => "Temperature Sensor",
            Sensor::Moisture => "Moisture Sensor",
            Sensor::Humidity => "Humidity Sensor",
            Sensor::Sunlight => "Sunlight Sensor",
        }
    }
}

/// The application state
struct CasaverdeApp {
    sensor_states: [bool; Sensor::ALL.len()], // ON/OFF for each sensor
    selected: usize,                          // which sensor is highlighted
    should_quit: bool,
}

impl CasaverdeApp {
    fn new() -> Self {
        Self {
            sensor_states: [false; Sensor::ALL.len()],
            selected: 0, // start on the first sensor
            should_quit: false,
        }
    }

    fn toggle_sensor(&mut self) {
        // flip the bool at `selected`
        self.sensor_states[self.selected] = !self.sensor_states[self.selected];
    }

    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected + 1 < self.sensor_states.len() {
            self.selected += 1;
        }
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}

fn main() -> io::Result<()> {
    // Enter raw mode + alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let mut app = CasaverdeApp::new();
    let res = run_app(&mut terminal, &mut app);

    // Cleanup
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut CasaverdeApp,
) -> io::Result<()> {
    // Initial draw
    terminal.draw(|frame| ui(frame, app))?;

    // Event loop
    loop {
        if app.should_quit {
            break;
        }

        // Poll for up to 200ms
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Up => app.move_up(),
                    KeyCode::Down => app.move_down(),
                    KeyCode::Enter => app.toggle_sensor(),
                    _ => {}
                }
            }
        }

        // Redraw
        terminal.draw(|frame| ui(frame, app))?;
    }

    Ok(())
}

fn ui(frame: &mut Frame, app: &CasaverdeApp) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),                              // Title box
            Constraint::Length((Sensor::ALL.len() as u16) + 2), // Sensors box
            Constraint::Length(3),                              // Status box
            Constraint::Min(0),                                 // Spacer
        ])
        .split(area);

    // 1) Title, centered
    let title = Paragraph::new("Casaverde")
        .block(Block::new().borders(Borders::ALL))
        .style(Style::default().fg(Color::Green))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // 2) Sensor list in a yellow box with centered header
    let items: Vec<ListItem> = Sensor::ALL
        .iter()
        .enumerate()
        .map(|(i, &sensor)| {
            let flag = if app.sensor_states[i] {
                "[ON]  "
            } else {
                "[OFF] "
            };
            ListItem::new(Span::raw(format!("{}{}", flag, sensor.name())))
        })
        .collect();

    let mut list_state = ratatui::widgets::ListState::default();
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

    // 3) Status line, centered
    let status = Paragraph::new(format!(
        "Toggle with Enter  (selected: {})",
        Sensor::ALL[app.selected].name()
    ))
    .block(Block::new().borders(Borders::ALL))
    .style(Style::default().fg(Color::Cyan))
    .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(status, chunks[2]);
}
