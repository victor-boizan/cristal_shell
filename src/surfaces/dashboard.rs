use super::Surface;
use super::SurfaceType;
use crate::messages::{Message, Update};
use crate::wayland::WaylandState;
use iced::Theme;
use iced::{
    Element, Length, Task, color,
    widget::{column, container, container::Style, row},
};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer, NewLayerShellSettings};
use wayland_client::protocol::wl_output;

pub struct Dashboard {}

impl Dashboard {
    pub fn new(_output: String) -> Self {
        Self {}
    }
}

impl Surface for Dashboard {
    fn layer_settings(&self, output: wl_output::WlOutput) -> NewLayerShellSettings {
        let width = 600;
        let height = (7 * 30) + (8 * 10) + (2 * 5) + (2 * 10);
        NewLayerShellSettings {
            size: Some((width as u32, height as u32)),
            exclusive_zone: None,
            anchor: Anchor::Top,
            layer: Layer::Overlay,
            margin: None,
            keyboard_interactivity: KeyboardInteractivity::None,
            events_transparent: false,
            namespace: Some("cristal_dashboard".to_string()),
            output_option: iced_layershell::reexport::OutputOption::Output(output.clone()),
        }
    }
    fn update(&mut self, _update: Update) -> Task<Message> {
        Task::none()
    }
    fn get_type(&self) -> SurfaceType {
        SurfaceType::Dashboard
    }
    fn view(&self) -> Element<'_, Message> {
        container(
            row![
                crate::widgets::dashboard_pages::Fetch::view(),
                crate::widgets::dashboard_pages::QuickSettings::view(),
                crate::widgets::dashboard_pages::NotificationHistory::view(),
            ]
            .spacing(10),
        )
        .padding(10)
        .style(|theme: &Theme| Style {
            text_color: Some(theme.clone().palette().text),
            background: Some(iced::Background::Color(theme.clone().palette().background)),
            border: iced::Border {
                color: color!(0x000000),
                width: 0.0,
                radius: iced::border::Radius::new(0).bottom(40),
            },
            shadow: iced::Shadow::default(),
            snap: true, //IDK WTF is this.
        })
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}
