use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App};

pub fn ui(frame:&mut Frame, app: &App) {
    let chunks = Layout::default() 
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let left_title = Paragraph::new(Text::styled(
            "Goal",
            Style::default().fg(Color::Green),
            ))
        .block(title_block.clone());

    let right_title = Paragraph::new(Text::styled(
            "Score",
            Style::default().fg(Color::Green),
            ))
        .block(title_block);

    frame.render_widget(left_title, chunks[0]);
    frame.render_widget(right_title, chunks[1]);
}
