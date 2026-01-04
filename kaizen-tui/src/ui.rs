use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App};

pub fn ui(frame:&mut Frame, app: &App) {

    let vertical_chunks = Layout::default() 
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Min(1),
            Constraint::Percentage(10),
        ])
        .split(frame.area());

    let horizontal_chunks = Layout::default() 
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
        ])
        .split(vertical_chunks[1]);

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

    frame.render_widget(left_title, horizontal_chunks[1]);
    frame.render_widget(right_title, horizontal_chunks[2]);
}
