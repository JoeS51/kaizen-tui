use std::io;

use ratatui::{
    DefaultTerminal,
    Frame,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen},
    ui::ui,
};

fn main() -> io::Result<()>   {
    ratatui::run(|terminal| run(terminal))
}

fn run(terminal:&mut DefaultTerminal) -> io::Result<()> {
    let mut app =  App::new();

    while !app.exit {
        terminal.draw(|frame| ui(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(())
                        }
                        KeyCode::Tab => {
                            app.toggle_screen();
                        }
                        KeyCode::Enter => {
                            if let CurrentScreen::Learning = &app.current_screen {
                                app.toggle_screen();
                                app.save_entry();
                            }
                        }
                        _ => {}
                    }
                }
                CurrentScreen::Learning => {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.toggle_screen();
                        }
                        KeyCode::Tab => {
                            app.toggle_screen();
                        }
                        KeyCode::Char(value) => {
                            app.key.push(value);
                        }
                        KeyCode::Backspace => {
                            app.key.pop();
                        }
                        KeyCode::Enter => {
                            if let CurrentScreen::Learning = &app.current_screen {
                                app.toggle_screen();
                                app.save_entry();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    return Ok(())
}
