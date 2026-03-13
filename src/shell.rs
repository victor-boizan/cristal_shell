use crate::{
    messages::{Message, Update},
    surfaces::{BoxedSurface, SurfaceType},
    wayland::outputs::Output,
};

use iced::{Element, Task};
use std::collections::HashMap;

pub struct Shell {
    pub output: Output,
    pub surfaces: HashMap<iced::window::Id, BoxedSurface>,
}

impl Shell {
    pub fn new(output: Output) -> (Self, Task<Message>) {
        let mut surfaces: HashMap<iced::window::Id, BoxedSurface> = HashMap::new();
        let mut tasks = Vec::new();

        for surface_type in SurfaceType::list_all() {
            let id = iced::window::Id::unique();
            let new_surface = surface_type.new(output.clone());
            let settings = new_surface.layer_settings(output.output.clone());
            surfaces.insert(id, new_surface);
            tasks.push(Task::done(Message::NewLayerShell { settings, id }));
        }
        (Self { surfaces, output }, Task::batch(tasks))
    }
    pub fn update(&mut self, update: Update) -> Task<Message> {
        let mut tasks = Vec::new();
        for (_, surface) in &mut self.surfaces {
            tasks.push(surface.update(update.clone()));
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
