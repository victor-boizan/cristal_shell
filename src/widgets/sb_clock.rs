use crate::messages::Message;
use chrono::Local;
use iced::widget::{column, container, text};
use iced::Element;
pub struct Clock {}

impl Clock {
    pub fn view(&self) -> Element<'_, Message> {
        let time = Local::now();
        container(column![
            text(time.format("%H").to_string()),
            text(time.format("%M").to_string()),
        ])
        .into()
    }
}
