use super::{
    button::{Button, ButtonState},
    ternary_selector::{Ternary, TernaryState},
};
use crate::messages::{Action, Message};
use iced::Theme;
use iced::{
    Color, Element, Length, color,
    widget::{
        column, container, container::Style as ConStyle, row, slider, slider::Style as SlidStyle,
    },
};

pub struct Slider {
    value: f32,
    button: Button,
}
impl Slider {
    pub fn new(icon: String) -> Self {
        Self {
            value: 50.0,
            button: Button::new_button(String::from(icon), 32.0, ButtonState::Active, Action::None),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        container(
            row![
                self.button.view(),
                container(
                    slider(0.0..=100.0, self.value, |_| Message::Test)
                        .style(|theme: &Theme, _| {
                            SlidStyle {
                                rail: Self::rail_style(theme),
                                handle: Self::handle_style(),
                            }
                        })
                        .height(30)
                )
                .style(|theme: &Theme| ConStyle {
                    text_color: Some(theme.clone().palette().text),
                    background: Some(iced::Background::Color(
                        theme.clone().extended_palette().background.strong.color,
                    )),
                    border: iced::Border {
                        color: color!(0x000000),
                        width: 0.0,
                        radius: iced::border::Radius::new(15),
                    },
                    shadow: iced::Shadow::default(),
                    snap: true, //IDK WTF is this.
                })
            ]
            .spacing(10),
        )
        .width(150)
        .center_y(30)
        .style(|theme: &Theme| ConStyle {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(
                theme.clone().extended_palette().background.weak.color,
            )),
            border: iced::Border {
                color: color!(0x000000),
                width: 0.0,
                radius: iced::border::Radius::new(15),
            },
            shadow: iced::Shadow::default(),
            snap: true, //IDK WTF is this.
        })
        .into()
    }
    fn rail_style(theme: &Theme) -> slider::Rail {
        slider::Rail {
            backgrounds: (
                iced::Background::Color(theme.clone().palette().text),
                iced::Background::Color(Color::TRANSPARENT),
            ),
            width: 30.0,
            border: iced::Border {
                color: color!(0x000000),
                width: 0.0,
                radius: iced::border::Radius::new(15),
            },
        }
    }
    fn handle_style() -> slider::Handle {
        slider::Handle {
            shape: slider::HandleShape::Circle { radius: 15.0 },
            background: iced::Background::Color(Color::TRANSPARENT),
            border_width: 0.0,
            border_color: color!(0x000000),
        }
    }
}
