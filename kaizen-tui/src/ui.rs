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

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let left_title = Paragraph::new(Text::styled(
            "Goal",
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
    for key in app.entries.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("{} : {}", key, app.entries.get(key).unwrap()),
                        Style::default().fg(Color::Green),
        ))));
    }

    let list = List::new(list_items);
    frame.render_widget(list, horizontal_chunks[1]);

    frame.render_widget(left_title, horizontal_chunks[1]);
    frame.render_widget(right_title, horizontal_chunks[2]);

    if let CurrentScreen::Learning = &app.current_screen {
        let popup_block = Block::default()
            .title("chat")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);
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
