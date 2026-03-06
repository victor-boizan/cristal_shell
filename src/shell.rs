use crate::{
    messages::{Message,Update},
    wayland::outputs::Output
};

use iced::{Element,Task};

pub struct Shell {}

impl Shell {
    pub fn new(_output: Output) -> (Self, Task<Message>) {
        (Self {},Task::none())
    }
    pub fn update(&mut self, _update: Update) -> Task<Message> {
        Task::none()
    }
    pub fn view(&self, _id: iced::window::Id) -> Element<'_, Message> {
	"".into()
    }
}
