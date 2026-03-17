use crate::messages::Action;
use crate::messages::Message;
use iced::color;
use iced::widget::button;
use iced::widget::button::Style;
use iced::widget::text;
use iced::Element;
use iced::Theme;

#[derive(Clone, PartialEq, Eq)]
enum ButtonState {
    Disable,
    Inactive,
    Active,
}

#[derive(Clone)]
pub struct Button {
    state: ButtonState,
    icon: String,
    action: Action,
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
            action: Action::Tick,
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let button_size: f32 = 20 as f32;
        let icon_size: f32 = button_size / f32::sqrt(2.0);
        let padding: f32 = (button_size - icon_size) / 2.0;

        button(
            text!("{}", self.icon)
                .center()
                .width(icon_size)
                .height(icon_size),
        )
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
        .width(button_size)
        .height(button_size)
        .into()
    }
}
