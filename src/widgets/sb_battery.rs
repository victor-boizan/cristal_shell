use crate::messages::Message;
use iced::widget::{container, text};
use iced::Element;
use std::fs;
use std::path::PathBuf;
pub struct Battery {
    lvl: String,
}

impl Battery {
    pub fn new() -> Self {
        let lvl =
            fs::read_to_string(PathBuf::from("/sys/class/power_supply/BAT0/capacity").as_path())
                .unwrap();
        Battery { lvl }
    }
    pub fn update(&mut self) {
        self.lvl =
            fs::read_to_string(PathBuf::from("/sys/class/power_supply/BAT0/capacity").as_path())
                .unwrap();
    }
    pub fn view(&self) -> Element<'_, Message> {
        container(text(self.lvl.clone())).into()
    }
}
