use super::Surface;
use super::SurfaceType;
use crate::{
    data::system::os::OS,
    data::wayland::{WaylandState, outputs::Output},
    messages::{Action, Message, Update},
    widgets::{
        button::{Button, ButtonState},
        sb_battery::Battery,
        sb_clock::Clock,
        sb_workspaces_list::WorkspacesList,
    },
};
use iced::Theme;
use iced::{
    Element, Length, Task, color,
    widget::{Space, column, container, container::Style},
};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer, NewLayerShellSettings};
use wayland_client::protocol::wl_output;

pub struct StatusBar {
    os_button: Button,
    workspaces_list: WorkspacesList,
    output: Output,
    clock: Clock,
    bat: Battery,
}

impl StatusBar {
    pub fn new(output: Output, wl_state: WaylandState) -> Self {
        let os_button = Button::new_button(
            OS::current().get_icon(),
            20.0,
            ButtonState::Active,
            Action::ToggleDashBoard,
        );
        let workspaces = wl_state.workspaces_for_output(output.clone());
        let clock = Clock::new();
        let bat = Battery::new();
        Self {
            os_button,
            workspaces_list: WorkspacesList::new(workspaces),
            output,
            clock,
            bat,
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
    fn update(&mut self, update: Update) -> Task<Message> {
        match update {
            Update::WaylandUpdate(state) => {
                let workspaces = state.workspaces_for_output(self.output.clone());
                self.workspaces_list.update(workspaces);
            }
            Update::Tick => {
                self.clock.update();
                self.bat.update();
            }
            Update::WaylandInit(_) => unreachable!(),
            _ => {}
        }
        Task::none()
    }
    fn get_type(&self) -> SurfaceType {
        SurfaceType::StatusBar
    }
    fn view(&self) -> Element<'_, Message> {
        container(column![
            self.os_button.view(),
            self.workspaces_list.view(),
            Space::new().height(Length::Fill),
            self.bat.view(),
            self.clock.view()
        ])
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
        .padding(iced::Padding::from([5, 5]))
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}
