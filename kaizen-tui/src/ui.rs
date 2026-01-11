use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

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

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0)
        ])
        .split(horizontal_chunks[1]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0)
        ])
        .split(horizontal_chunks[2]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let left_title = Paragraph::new(Text::styled(
            "Learnt",
            Style::default().fg(Color::Green),
            ))
        .block(title_block.clone())
        .centered();

    let right_title = Paragraph::new(Text::styled(
            "Score",
            Style::default().fg(Color::Green),
            ))
        .block(title_block)
        .centered();

    let mut list_items = Vec::<ListItem>::new();
    let mut list_items_vals = Vec::<ListItem>::new();
    for key in app.entries.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("{: <25}", key),
                        Style::default().fg(Color::Green),
        ))));
    }
    for key in app.entries.keys() {
        list_items_vals.push(ListItem::new(Line::from(Span::styled(
                        format!("{: <25}", app.entries.get(key).unwrap()),
                        Style::default().fg(Color::Green),
        ))));
    }

    let list = List::new(list_items).block(Block::default().borders(Borders::ALL));
    let list_val = List::new(list_items_vals).block(Block::default().borders(Borders::ALL));
    frame.render_widget(list, left_chunks[1]);
    frame.render_widget(list_val, right_chunks[1]);

    frame.render_widget(left_title, horizontal_chunks[1]);
    frame.render_widget(right_title, horizontal_chunks[2]);

    if let CurrentScreen::Learning = &app.current_screen {
        let popup_block = Block::default()
            .title("chat")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let key_text = Paragraph::new(app.key.clone());
        frame.render_widget(popup_block, area);
        frame.render_widget(key_text, popup_chunks[0]);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2), 
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2), 
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
