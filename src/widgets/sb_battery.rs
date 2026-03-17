use crate::messages::Message;
use iced::color;
use iced::widget::{container, container::Style, text};
use iced::Element;
use iced::Theme;
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
        container(text(self.lvl.clone()))
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
            .height(20)
            .into()
    }
}
