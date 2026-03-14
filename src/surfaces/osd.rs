use super::Surface;
use super::SurfaceType;
use crate::messages::{Message, Update};
use crate::wayland::WaylandState;
use iced::Theme;
use iced::{
    color,
    widget::{container, container::Style},
    Element, Length, Task,
};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer, NewLayerShellSettings};
use wayland_client::protocol::wl_output;

pub struct OSD {}

impl OSD {
    pub fn new(_output: String) -> Self {
        Self {}
    }
}

impl Surface for OSD {
    fn layer_settings(&self, output: wl_output::WlOutput) -> NewLayerShellSettings {
        let width = 30;
        NewLayerShellSettings {
            size: Some((width as u32, 300)),
            exclusive_zone: Some(-1), // make it ignore exclusive zones
            anchor: Anchor::Left,
            layer: Layer::Overlay,
            margin: None,
            keyboard_interactivity: KeyboardInteractivity::None,
            events_transparent: false,
            namespace: Some("cristal_osd".to_string()),
            output_option: iced_layershell::reexport::OutputOption::Output(output.clone()),
        }
    }
    fn update(&mut self, _update: Update, _wl_state: WaylandState) -> Task<Message> {
        Task::none()
    }
    fn get_type(&self) -> SurfaceType {
        SurfaceType::OSD
    }
    fn view(&self) -> Element<'_, Message> {
        container("O")
            .style(|theme: &Theme| Style {
                text_color: Some(theme.clone().palette().text),
                background: Some(iced::Background::Color(theme.clone().palette().background)),
                border: iced::Border {
                    color: color!(0x000000),
                    width: 0.0,
                    radius: iced::border::Radius::new(0),
                },
                shadow: iced::Shadow::default(),
                snap: true, //IDK WTF is this.
            })
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
