use iced::{Element, Task};
use iced::{Color, color};
use wayland_client::Connection;
use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    messages::{Message,Update},
    wayland::outputs::Output,
    shell::Shell
};

pub struct Session {
    shells: HashMap<Output, Shell> //each output will have its own shell
}

impl Session {
    pub fn new(conn: Arc<Connection>) -> (Self, Task<Message>) {
        let outputs = crate::wayland::outputs::Output::get_all(conn);

        let mut shells: HashMap<Output, Shell> = HashMap::new();
        let mut tasks: Vec<Task<Message>> = Vec::new();

        for output in outputs {
            let (shell, task) = Shell::new(output.clone());
            shells.insert(output, shell);
            tasks.push(task);
        }
        (Self { shells }, Task::batch(tasks))
    }
    pub fn style(&self, _theme: &iced::Theme) -> iced::theme::Style {
        iced::theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: color!(0xffffff),
            }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Update(update) => {
		let mut tasks = Vec::new();
		for (_, shell) in &mut self.shells {
		    tasks.push(shell.update(update.clone()));
		}
		Task::batch(tasks)
	    },
	    _ => Task::none()
	}
    }
    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(std::time::Duration::from_secs(1))
            .map(|_| Message::Update(Update::Tick))
    }
    pub fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
        for (_, shell) in &self.shells {
            if shell.surfaces.get(&id).is_none() { continue; }
            return shell.view(id);
	}
	"".into()
    }
}
