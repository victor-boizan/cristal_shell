
use wayland_client::protocol::wl_output;
use crate::{
    messages::{Message,Update},
    wayland::outputs::Output
};

use iced_layershell::reexport::NewLayerShellSettings;

use iced::{Element,Task};

// Surface modules
mod status_bar;
use status_bar::StatusBar;
mod corners;
use corners::Corners;

// list all Surfaces Types
#[derive(Eq,PartialEq)]
pub enum SurfaceType {
    StatusBar,
    Corners,
}

pub type BoxedSurface = Box<dyn Surface>;

pub trait Surface {
    fn layer_settings(&self, output: wl_output::WlOutput) -> NewLayerShellSettings;
    fn update(&mut self, update: Update) -> Task<Message>;
    fn view(&self) -> Element<'_,Message>;
    fn get_type(&self) -> SurfaceType;
}

impl SurfaceType {
    pub fn list_all() -> Vec<Self> {
       vec![Self::StatusBar,Self::Corners] 
    }
    pub fn new(&self, output: Output) -> BoxedSurface {
        match &self {
            Self::StatusBar  => Box::new(StatusBar::new(output.name)) as BoxedSurface,
            Self::Corners    => Box::new(Corners::new()) as BoxedSurface,
        }
    }
}
