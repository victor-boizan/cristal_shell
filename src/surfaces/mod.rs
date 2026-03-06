
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
mod background;
use background::Background;
// list all Surfaces Types
#[derive(Eq,PartialEq,Clone)]
pub enum SurfaceType {
    StatusBar,
    Corners,
    WorkspaceBackground,
    OverviewBackground,
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
	vec![
	    Self::WorkspaceBackground,
	    Self::OverviewBackground,
	    Self::StatusBar,
	    Self::Corners,
	] 
    }
    pub fn new(&self, output: Output) -> BoxedSurface {
        match &self {
            Self::StatusBar  => Box::new(StatusBar::new(output.name)) as BoxedSurface,
            Self::Corners    => Box::new(Corners::new()) as BoxedSurface,
	    Self::WorkspaceBackground => Box::new(Background::new(self.clone())) as BoxedSurface,
	    Self::OverviewBackground  => Box::new(Background::new(self.clone())) as BoxedSurface,
        }
    }
}
