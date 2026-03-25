use super::button::{Button, ButtonState};
use crate::messages::{Action, Message};
use iced::{
    Element, Theme, color,
    widget::{container, container::Style, row},
};

#[derive(Clone, PartialEq, Eq)]
pub enum TernaryState {
    Left,
    Center,
    Right,
}

#[derive(Clone)]
pub struct Ternary {
    state: TernaryState,
    button_l: Button,
    button_c: Button,
    button_r: Button,
}

impl Ternary {
    pub fn new(icons: [String; 3], state: TernaryState, actions: [Action; 3]) -> Self {
        Self {
            state,
            button_l: Button::new_button(
                icons[0].clone(),
                20.0,
                ButtonState::Active,
                actions[0].clone(),
            ),
            button_c: Button::new_button(
                icons[1].clone(),
                20.0,
                ButtonState::Inactive,
                actions[1].clone(),
            ),
            button_r: Button::new_button(
                icons[2].clone(),
                20.0,
                ButtonState::Inactive,
                actions[2].clone(),
            ),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        container(
            row![
                self.button_l.view(),
                self.button_c.view(),
                self.button_r.view()
            ]
            .spacing(2.5),
        )
        .style(|theme: &Theme| Style {
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
        .center_y(30)
        .center_x(70)
        .into()
    }
}
