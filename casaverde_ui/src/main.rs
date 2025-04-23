use crossterm::{
    event::{self, Event, KeyCode, MouseButton, MouseEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::io::{self, Stdout};

struct GreenhouseApp {
    button_pressed: bool,
    should_quit: bool,
}

impl GreenhouseApp {
    fn new() -> Self {
        Self {
            button_pressed: false,
            should_quit: false,
        }
    }

    fn toggle_sensor(&mut self) {
        self.button_pressed = !self.button_pressed;
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}

fn main() -> io::Result<()> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let mut app = GreenhouseApp::new();
    let res = run_app(&mut terminal, &mut app);

    // Cleanup terminal
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut GreenhouseApp,
) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| ui(frame, app))?;
        handle_events(app)?;
    }
    Ok(())
}

fn ui(frame: &mut Frame, app: &GreenhouseApp) {
    let area = frame.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(5), // Button
            Constraint::Length(3), // Status
            Constraint::Min(0),    // Spacer
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Greenhouse Control")
        .style(Style::default().fg(Color::Green))
        .block(Block::new().borders(Borders::ALL));
    frame.render_widget(title, layout[0]);

    // Toggle Sensor Button
    let button_text = "[ Toggle Sensor ]";
    let button = Paragraph::new(button_text)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .bg(if app.button_pressed {
                    Color::DarkGray
                } else {
                    Color::Reset
                }),
        )
        .block(Block::new().borders(Borders::ALL).title("Button"));
    frame.render_widget(button, layout[1]);

    // Status
    let status = Paragraph::new(if app.button_pressed {
        "Sensor: ON"
    } else {
        "Sensor: OFF"
    })
    .style(Style::default().fg(Color::Cyan))
    .block(Block::new().borders(Borders::ALL));
    frame.render_widget(status, layout[2]);
}

fn handle_events(app: &mut GreenhouseApp) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        match event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q') {
                    app.quit();
                }
            }
            Event::Mouse(mouse) => {
                if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                    // Check if click is within button area (approx. y=4-8 for button)
                    if mouse.row >= 4 && mouse.row <= 8 {
                        app.toggle_sensor();
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
