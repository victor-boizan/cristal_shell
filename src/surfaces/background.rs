use super::Surface;
use crate::{
    messages::{Message, Update},
    surfaces::SurfaceType,
};
use iced::{Element, Task};
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer, NewLayerShellSettings};
use std::path::PathBuf;
use wayland_client::protocol::wl_output;

#[derive(Clone)]
pub struct Background {
    background_type: SurfaceType,
    image: image::Handle,
}

use iced::{
    Length,
    widget::{container, image},
};

impl Surface for Background {
    fn layer_settings(&self, output: wl_output::WlOutput) -> NewLayerShellSettings {
        let namespace = match self.background_type {
            SurfaceType::WorkspaceBackground => Some(String::from("Cristal_Background")),
            SurfaceType::OverviewBackground => Some(String::from("Cristal_Overview")),
            _ => panic!(
                "Background can only be a Workspace_Background surface or an Overview_Background surface"
            ),
        };

        NewLayerShellSettings {
            size: Some((0, 0)),
            exclusive_zone: Some(-1),
            anchor: Anchor::all(),
            layer: Layer::Background,
            margin: Some((0, 0, 0, 30)), //Up,Right,Down,Left
            keyboard_interactivity: KeyboardInteractivity::None,
            events_transparent: true,
            namespace,
            output_option: iced_layershell::reexport::OutputOption::Output(output.clone()),
        }
    }
    fn get_type(&self) -> SurfaceType {
        self.background_type.clone()
    }
    fn update(&mut self, _update: Update) -> Task<Message> {
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let image_widget = image::Image::new(&self.image)
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Cover)
            .border_radius(20);

        container(image_widget)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
impl Background {
    pub fn new(background_type: SurfaceType) -> Self {
        let home = std::env::home_dir().unwrap();
        let image: image::Handle = match background_type {
            SurfaceType::WorkspaceBackground => {
                image::Handle::from_path(PathBuf::from(format!("{}/Wallpaper.jpg", home.display())))
            }
            SurfaceType::OverviewBackground => {
                image::Handle::from_path(PathBuf::from(format!("{}/Overview.jpg", home.display())))
            }
            _ => panic!(
                "Background can only be a Workspace_Background surface or an Overview_Background surface"
            ),
        };
        Self {
            image,
            background_type,
        }
    }
}
