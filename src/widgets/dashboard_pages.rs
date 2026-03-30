use super::{
    button::{Button, ButtonState},
    slider::Slider,
    ternary_selector::{Ternary, TernaryState},
};
use crate::messages::{Action, Message};
use iced::Theme;
use iced::{
    Color, Element, Length, color,
    widget::{column, container, container::Style, row},
};

pub struct Fetch {}

impl Fetch {
    pub fn view() -> Element<'static, Message> {
        container(
            column![
                row![
                    large_logo(),
                    column![label(), label(), label(), label()].spacing(10 / 3)
                ]
                .spacing(10),
                //row![slider()].spacing(10),
                //row![slider()].spacing(10),
                //row![slider()].spacing(10),
                //row![slider()].spacing(10),
                row![pill(), pill()].spacing(10),
            ]
            .width(150)
            .spacing(10),
        )
        .padding(15)
        .style(|theme: &Theme| Style {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(color!(0xAA8888))),
            border: iced::Border {
                color: color!(0xffffff),
                width: 5.0,
                radius: iced::border::Radius::new(30),
            },
            shadow: iced::Shadow::default(),
            snap: true, //IDK WTF is this.
        })
        .into()
    }
}

pub struct QuickSettings {
    screenshot_icon: Button,
    game_mode_icon: Button,
    quick_note_icon: Button,
    notifications_icon: Button,
    rotation_icon: Button,
    coffee_icon: Button,
    night_icon: Button,
    theme_icon: Button,
    anc_selector: Ternary,
    pwr_profile_selector: Ternary,
    vol_slider: Slider,
    backlight_slider: Slider,
}

impl QuickSettings {
    pub fn new() -> Self {
        Self {
            screenshot_icon: Button::new_button(
                String::from("󰹑"),
                30.0,
                ButtonState::Active,
                Action::None,
            ),
            game_mode_icon: Button::new_button(
                String::from("󰊖"),
                30.0,
                ButtonState::Active,
                Action::None,
            ),
            quick_note_icon: Button::new_button(
                String::from("󰎚"),
                30.0,
                ButtonState::Active,
                Action::None,
            ),
            notifications_icon: Button::new_button(
                String::from("󰂚"),
                30.0,
                ButtonState::Active,
                Action::None,
            ),
            rotation_icon: Button::new_button(
                String::from("󰑵"),
                30.0,
                ButtonState::Active,
                Action::None,
            ),
            coffee_icon: Button::new_button(
                String::from("󰅶"),
                30.0,
                ButtonState::Active,
                Action::None,
            ),
            night_icon: Button::new_button(
                String::from("󰖔"),
                30.0,
                ButtonState::Active,
                Action::None,
            ),
            theme_icon: Button::new_button(
                String::from("󱎖"),
                30.0,
                ButtonState::Active,
                Action::ToggleTheme,
            ),
            anc_selector: Ternary::new(
                [String::from(""), String::from("󰾆"), String::from("󰗅")],
                TernaryState::Left,
                [Action::None, Action::None, Action::None],
            ),
            pwr_profile_selector: Ternary::new(
                [String::from("󰾆"), String::from("󰾅"), String::from("󰓅")],
                TernaryState::Left,
                [Action::None, Action::None, Action::None],
            ),
            vol_slider: Slider::new(String::from("")),
            backlight_slider: Slider::new(String::from("󰳲")),
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        container(
            column![
                row![pill(), pill()].spacing(10), //user,network
                row![
                    self.screenshot_icon.view(),
                    self.pwr_profile_selector.view(),
                    self.game_mode_icon.view()
                ]
                .spacing(10),
                row![
                    pill(),
                    self.quick_note_icon.view(),
                    self.notifications_icon.view()
                ]
                .spacing(10), //pomodoro
                self.backlight_slider.view(),
                row![
                    self.night_icon.view(),
                    self.theme_icon.view(),
                    self.rotation_icon.view(),
                    self.coffee_icon.view()
                ]
                .spacing(10),
                row![
                    pill(),                   //output
                    self.anc_selector.view(), //anc
                ]
                .spacing(10),
                self.vol_slider.view()
            ]
            .spacing(10),
        )
        .padding(15)
        .style(|theme: &Theme| Style {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(
                theme.clone().extended_palette().background.weakest.color,
            )),
            border: iced::Border {
                color: theme.clone().palette().text,
                width: 5.0,
                radius: iced::border::Radius::new(30),
            },
            shadow: iced::Shadow::default(),
            snap: true, //IDK WTF is this.
        })
        .into()
    }
}

pub struct NotificationHistory {}
impl NotificationHistory {
    pub fn view() -> Element<'static, Message> {
        container(column![notification(), notification(), notification()].spacing(10))
            .height(Length::Fill)
            .padding(15)
            .style(|theme: &Theme| Style {
                text_color: Some(theme.clone().palette().text),
                background: Some(iced::Background::Color(color!(0xAA8888))),
                border: iced::Border {
                    color: color!(0xffffff),
                    width: 5.0,
                    radius: iced::border::Radius::new(30),
                },
                shadow: iced::Shadow::default(),
                snap: true, //IDK WTF is this.
            })
            .into()
    }
}

fn notification() -> Element<'static, Message> {
    container("")
        .width(150)
        .height(70)
        .style(|theme: &Theme| Style {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(color!(0x888888))),
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
fn large_logo() -> Element<'static, Message> {
    container("")
        .width(70)
        .height(70)
        .style(|theme: &Theme| Style {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(color!(0x888888))),
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
fn label() -> Element<'static, Message> {
    container("")
        .width(70)
        .height(15)
        .style(|theme: &Theme| Style {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(color!(0x888888))),
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
fn pill() -> Element<'static, Message> {
    container("")
        .width(70)
        .height(30)
        .style(|theme: &Theme| Style {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(color!(0x888888))),
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

fn vpill() -> Element<'static, Message> {
    container("")
        .height(70)
        .width(30)
        .style(|theme: &Theme| Style {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(color!(0x888888))),
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
