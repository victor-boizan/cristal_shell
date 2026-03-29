use crate::messages::Message;
use chrono::{DateTime, Local};
use iced::Element;
use iced::Theme;
use iced::color;
use iced::widget::{column, container, container::Style, text};
pub struct Clock {
    time: DateTime<Local>,
}

impl Clock {
    pub fn new() -> Self {
        Self { time: Local::now() }
    }
    pub fn update(&mut self) {
        self.time = Local::now();
    }
    pub fn view(&self) -> Element<'_, Message> {
        container(column![
            text(self.time.format("%H").to_string()),
            text(self.time.format("%M").to_string()),
        ])
        .style(|theme: &Theme| Style {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(
                theme.clone().extended_palette().background.weak.color,
            )),
            border: iced::Border {
                color: color!(0x000000),
                width: 0.0,
                radius: iced::border::Radius::new(10),
            },
            shadow: iced::Shadow::default(),
            snap: true, //IDK WTF is this.
        })
        .into()
    }
}
