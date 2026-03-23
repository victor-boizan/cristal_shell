use crate::messages::Message;
use iced::Theme;
use iced::{
    Element, Length, color,
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
                row![slider()].spacing(10),
                row![slider()].spacing(10),
                row![slider()].spacing(10),
                row![slider()].spacing(10),
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

pub struct QuickSettings {}

impl QuickSettings {
    pub fn view() -> Element<'static, Message> {
        container(
            column![
                row![pill(), pill()].spacing(10),
                row![icon(), pill(), icon()].spacing(10),
                row![pill(), icon(), icon()].spacing(10),
                slider(),
                row![
                    vpill(),
                    vpill(),
                    column![row![icon(), icon()].spacing(10), pill()].spacing(10)
                ]
                .spacing(10),
                slider(),
            ]
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
fn slider() -> Element<'static, Message> {
    container("")
        .width(150)
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
fn icon() -> Element<'static, Message> {
    container("")
        .width(30)
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
