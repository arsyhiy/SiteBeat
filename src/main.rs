use std::{
    collections::HashMap,
    io,
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};

mod files;

fn get_connection_status(sites: HashMap<String, String>) -> HashMap<String, String> {
    let mut results = HashMap::new();

    for (name, url) in &sites {
        let status = match reqwest::blocking::get(url) {
            Ok(response) => response.status().to_string(),
            Err(e) => format!("Ошибка: {:?}", e),
        };

        results.insert(name.clone(), status);
    }

    results
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(
        stdout,
        EnterAlternateScreen
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;

    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("{err}");
    }

    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {

    let sites = files::load_sites();
    let connection_status = get_connection_status(sites);

    loop {
        terminal.draw(|frame| {
            let size = frame.area();

            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(5),
                    Constraint::Length(2),
                ])
                .split(size);

            draw_connection_status(frame, vertical[0], &connection_status);
            draw_footer(frame, vertical[1]);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,

                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn draw_connection_status(
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    sites: &HashMap<String, String>,
) {
    let header = Row::new(vec![
        "SITE",
        "STATUS",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .bottom_margin(1);

    let rows = sites.iter().map(|(name, status)| {
        Row::new(vec![
            name.clone(),
            status.clone(),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Connection Status ")
            .borders(Borders::ALL),
    )
    .row_highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_widget(table, area);
}

fn draw_footer(
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
) {
    let footer = Paragraph::new(
        " q: Quit ",
    )
    .block(Block::default().borders(Borders::ALL))
    .style(
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_widget(footer, area);
}
