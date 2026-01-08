use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Learning,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub exit: bool,
    pub entries: HashMap<String, String>,
    pub key: String,
    pub value: String,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            exit: false,
            entries: HashMap::new(),
            key: String::from("key"),
            value: String::from("value"),
        }
    }

    pub fn toggle_screen(&mut self) {
        match self.current_screen {
            CurrentScreen::Main => self.current_screen = CurrentScreen::Learning,
            CurrentScreen::Learning => self.current_screen = CurrentScreen::Main,
        }
    }

    pub fn save_entry(&mut self) {
        self.entries.insert(self.key.clone(), self.value.clone());
        println!("test");
        self.key = String::new();
        self.value = String::new();
    }
}
