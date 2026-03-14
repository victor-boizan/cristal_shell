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

pub struct Runner {}

impl Runner {
    pub fn new(_output: String) -> Self {
        Self {}
    }
}

impl Surface for Runner {
    fn layer_settings(&self, output: wl_output::WlOutput) -> NewLayerShellSettings {
        let width = 300;
        let height = 200;
        NewLayerShellSettings {
            size: Some((width as u32, height as u32)),
            exclusive_zone: None,
            anchor: Anchor::Left | Anchor::Bottom | Anchor::Right,
            layer: Layer::Overlay,
            margin: Some((0, 300, 0, 300)), //Up,Right,Down,Left
            keyboard_interactivity: KeyboardInteractivity::None,
            events_transparent: false,
            namespace: Some("cristal_runner".to_string()),
            output_option: iced_layershell::reexport::OutputOption::Output(output.clone()),
        }
    }
    fn update(&mut self, _update: Update, _wl_state: WaylandState) -> Task<Message> {
        Task::none()
    }
    fn get_type(&self) -> SurfaceType {
        SurfaceType::Runner
    }
    fn view(&self) -> Element<'_, Message> {
        container("")
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
