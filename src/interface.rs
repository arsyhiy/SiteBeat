use std::collections::HashMap;

use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
};

pub fn draw_connection_status(
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    sites: &HashMap<String, String>,
) {
    let header = Row::new(vec!["SITE", "STATUS"])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let rows = sites
        .iter()
        .map(|(name, status)| Row::new(vec![name.clone(), status.clone()]));

    let table = Table::new(
        rows,
        [Constraint::Percentage(30), Constraint::Percentage(70)],
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

pub fn draw_footer(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
    let footer = Paragraph::new(" q: Quit ")
        .block(Block::default().borders(Borders::ALL))
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(footer, area);
}
