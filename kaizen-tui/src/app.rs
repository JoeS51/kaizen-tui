pub enum CurrentScreen {
    Main,
    Learning,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            exit: false,
        }
    }

    pub fn toggle_screen(&mut self) {
        match self.current_screen {
            CurrentScreen::Main => self.current_screen = CurrentScreen::Learning,
            CurrentScreen::Learning => self.current_screen = CurrentScreen::Main,
        }
    }
}
