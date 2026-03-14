use crate::messages::Action;
use crate::messages::Message;
use iced::color;
use iced::widget::button;
use iced::widget::button::Style;
use iced::widget::text;
use iced::Element;
use iced::Theme;

#[derive(Clone)]
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
    pub fn new(icon: String) -> Self {
        Self {
            state: ButtonState::Disable,
            icon,
            action: Action::Tick,
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        button(text!("{}", self.icon))
            .style(|theme: &Theme, _| Style {
                text_color: theme.clone().palette().text,
                background: Some(iced::Background::Color(color!(0x000000))),
                border: iced::Border {
                    color: color!(0x000000),
                    width: 0.0,
                    radius: iced::border::Radius::new(30),
                },
                shadow: iced::Shadow::default(),
                snap: true, //IDK WTF is this.
            })
            .width(30)
            .height(30)
            .into()
    }
}
