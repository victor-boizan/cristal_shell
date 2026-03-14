use crate::{
    messages::{Message, Update},
    surfaces::{BoxedSurface, SurfaceType},
    wayland::outputs::Output,
    wayland::WaylandState,
};

use iced::{Element, Task};
use std::collections::HashMap;

pub struct Shell {
    pub output: Output,
    wl_state: WaylandState,
    pub surfaces: HashMap<iced::window::Id, BoxedSurface>,
}

impl Shell {
    pub fn new(output: Output, wl_state: WaylandState) -> (Self, Task<Message>) {
        let mut surfaces: HashMap<iced::window::Id, BoxedSurface> = HashMap::new();
        let mut tasks = Vec::new();

        for surface_type in SurfaceType::list_all() {
            let id = iced::window::Id::unique();
            let new_surface = surface_type.new(output.clone(), wl_state.clone());
            let settings = new_surface.layer_settings(output.output.clone());
            surfaces.insert(id, new_surface);
            tasks.push(Task::done(Message::NewLayerShell { settings, id }));
        }
        (
            Self {
                surfaces,
                output,
                wl_state,
            },
            Task::batch(tasks),
        )
    }
    pub fn update(&mut self, update: Update, wl_state: WaylandState) -> Task<Message> {
        self.wl_state = wl_state.clone();
        let mut tasks = Vec::new();
        for (_, surface) in &mut self.surfaces {
            tasks.push(surface.update(update.clone(), wl_state.clone()));
        }
        Task::batch(tasks)
    }
    pub fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
        match self.surfaces.get(&id) {
            Some(surface) => surface.view(),
            None => "".into(),
        }
    }
}
