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
}
