use crate::messages::Message;
use chrono::Local;
use iced::color;
use iced::widget::{column, container, container::Style, text};
use iced::Element;
use iced::Theme;
pub struct Clock {}

impl Clock {
    pub fn view(&self) -> Element<'_, Message> {
        let time = Local::now();
        container(column![
            text(time.format("%H").to_string()),
            text(time.format("%M").to_string()),
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
