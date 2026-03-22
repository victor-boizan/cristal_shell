use crate::{
    messages::{Message, Update},
    surfaces::{BoxedSurface, SurfaceType},
    wayland::WaylandState,
    wayland::outputs::Output,
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
        println!("shell created");
        (
            Self {
                surfaces,
                output,
                wl_state,
            },
            Task::batch(tasks),
        )
    }
    //pub fn update(&mut self, update: Update, wl_state: WaylandState) -> Task<Message> {
    pub fn update(&mut self, update: Update) -> Task<Message> {
        match update {
            Update::ToggleDashBoard => {
                let mut dashboard_id: Option<iced::window::Id> = None;
                for (id, surface) in &mut self.surfaces {
                    if SurfaceType::Dashboard == surface.get_type() {
                        dashboard_id = Some(id.clone());
                    }
                }
                if let Some(id) = dashboard_id {
                    println!("Close DashBoard");
                    self.surfaces.remove(&id);
                    return iced::window::close::<Message>(id.clone());
                }
                println!("Open New DashBoard");
                let id = iced::window::Id::unique();
                let surface_type = SurfaceType::Dashboard;
                let new_surface = surface_type.new(self.output.clone(), self.wl_state.clone());
                let settings = new_surface.layer_settings(self.output.output.clone());
                self.surfaces.insert(id, new_surface);
                Task::done(Message::NewLayerShell { settings, id })
            }
            .into(),
            Update::WaylandInit(_) => {
                unreachable!()
            }
            _ => {
                let mut tasks = Vec::new();
                for (_, surface) in &mut self.surfaces {
                    //tasks.push(surface.update(update.clone(), wl_state.clone()));
                    tasks.push(surface.update(update.clone()));
                }
                Task::batch(tasks)
            }
        }
    }
    pub fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
        match self.surfaces.get(&id) {
            Some(surface) => surface.view(),
            None => "".into(),
        }
    }
}
