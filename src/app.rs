use std::{collections::HashMap, io, sync::mpsc, thread, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
};

use crate::files;
use crate::interface;

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let sites = files::load_sites();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for (name, url) in sites {
            let status = match reqwest::blocking::get(&url) {
                Ok(response) => response.status().to_string(),
                Err(e) => format!("Ошибка: {:?}", e),
            };

            tx.send((name, status)).unwrap();
        }
    });

    let mut connection_status = HashMap::new();

    loop {
        while let Ok((name, status)) = rx.try_recv() {
            connection_status.insert(name, status);
        }

        terminal.draw(|frame| {
            let size = frame.area();

            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(5), Constraint::Length(2)])
                .split(size);

            interface::draw_connection_status(frame, vertical[0], &connection_status);
            interface::draw_footer(frame, vertical[1]);
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
