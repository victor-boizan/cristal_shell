use crate::messages::{Action, Message};
use iced::{
    Element, Theme, color,
    widget::{button, button::Style, text},
};

#[derive(Clone, PartialEq, Eq)]
pub enum ButtonState {
    //Disable,
    Inactive,
    Active,
}

#[derive(Clone)]
pub struct Button {
    state: ButtonState,
    icon: String,
    action: Action,
    pub size: f32,
}

impl Button {
    pub fn new(icon: String, is_primary: bool) -> Self {
        let state = match is_primary {
            true => ButtonState::Active,
            false => ButtonState::Inactive,
        };
        Self {
            state,
            icon,
            action: Action::None,
            size: 20.0,
        }
    }
    pub fn new_button(icon: String, size: f32, state: ButtonState, action: Action) -> Self {
        Self {
            state,
            icon,
            action,
            size,
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let icon_size: f32 = self.size / f32::sqrt(2.0);
        let padding: f32 = (self.size - icon_size) / 2.0;

        button(
            text!("{}", self.icon)
                .center()
                .width(icon_size)
                .height(icon_size),
        )
        .on_press(Message::Action(self.action.clone()))
        .style(|theme: &Theme, _status| Style {
            text_color: theme.clone().palette().text,
            background: Some(iced::Background::Color(
                if self.state == ButtonState::Active {
                    // check if the button is "primary"
                    theme.clone().extended_palette().background.strong.color
                } else {
                    theme.clone().extended_palette().background.weak.color
                },
            )),
            border: iced::Border {
                color: color!(0x000000),
                width: 0.0,
                radius: iced::border::Radius::new(30),
            },
            shadow: iced::Shadow::default(),
            snap: true, //IDK WTF is this.
        })
        .padding(padding)
        .width(self.size)
        .height(self.size)
        .into()
    }
}
