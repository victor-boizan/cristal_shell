use super::Surface;
use super::SurfaceType;
use crate::messages::{Message, Update};
use crate::wayland::outputs::Output;
use crate::wayland::WaylandState;
use crate::widgets::sb_workspaces_list::WorkspacesList;
use iced::Theme;
use iced::{
    color,
    widget::{column, container, container::Style},
    Element, Length, Task,
};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer, NewLayerShellSettings};
use wayland_client::protocol::wl_output;

pub struct StatusBar {
    workspaces_list: WorkspacesList,
    output: Output,
}

impl StatusBar {
    pub fn new(output: Output, wl_state: WaylandState) -> Self {
        let workspaces = wl_state.workspaces_for_output(output.clone());
        Self {
            workspaces_list: WorkspacesList::new(workspaces),
            output,
        }
    }
}

impl Surface for StatusBar {
    fn layer_settings(&self, output: wl_output::WlOutput) -> NewLayerShellSettings {
        let width = 30;
        NewLayerShellSettings {
            size: Some((width as u32, 0)),
            exclusive_zone: Some(width),
            anchor: Anchor::Left | Anchor::Top | Anchor::Bottom,
            layer: Layer::Top,
            margin: None,
            keyboard_interactivity: KeyboardInteractivity::None,
            events_transparent: false,
            namespace: Some("cristal_status_bar".to_string()),
            output_option: iced_layershell::reexport::OutputOption::Output(output.clone()),
        }
    }
    fn update(&mut self, _update: Update, wl_state: WaylandState) -> Task<Message> {
        let workspaces = wl_state.workspaces_for_output(self.output.clone());
        self.workspaces_list.update(workspaces);
        Task::none()
    }
    fn get_type(&self) -> SurfaceType {
        SurfaceType::StatusBar
    }
    fn view(&self) -> Element<'_, Message> {
        container(column![self.workspaces_list.view(), ""])
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
