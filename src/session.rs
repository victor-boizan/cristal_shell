use iced::{Element, Task};
use iced::{Color, color};

use crate::messages::Message;

pub struct Session {
}

impl Session {
    pub fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }
    pub fn style(&self, _theme: &iced::Theme) -> iced::theme::Style {
        iced::theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: color!(0xffffff),
            }
    }
    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }
    pub fn view(&self, _id: iced::window::Id) -> Element<'_, Message> {
        "".into()
    }
}
