use iced::{color, Color};
use iced::{Element, Task};
use std::collections::HashMap;
use std::sync::Arc;
use wayland_client::Connection;

use crate::{
    messages::{Message, Update},
    shell::Shell,
    wayland::outputs::Output,
};

pub struct Session {
    shells: HashMap<Output, Shell>, //each output will have its own shell
    conn: Arc<Connection>,
}

impl Session {
    pub fn new(conn: Arc<Connection>) -> (Self, Task<Message>) {
        let shells: HashMap<Output, Shell> = HashMap::new();

        (Self { shells, conn }, Task::none())
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
                if let Update::WaylandInit(state) = update {
                    println!("we got an init, their is {} outputs", state.outputs.len());
                    let outputs = state.outputs.clone();

                    self.shells = HashMap::new();
                    let mut tasks: Vec<Task<Message>> = Vec::new();

                    for output in outputs {
                        let (shell, task) = Shell::new(output.clone(), state.clone());
                        self.shells.insert(output, shell);
                        tasks.push(task);
                    }
                    return Task::batch(tasks);
                }
                let mut tasks = Vec::new();
                for (_, shell) in &mut self.shells {
                    tasks.push(shell.update(update.clone()));
                }
                Task::batch(tasks)
            }
            _ => Task::none(),
        }
    }
    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch([
            iced::time::every(std::time::Duration::from_secs(1))
                .map(|_| Message::Update(Update::Tick)),
            crate::wayland::WaylandState::subscription(self.conn.clone()).map(|msg| msg),
        ])
    }
    pub fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
        for (_, shell) in &self.shells {
            if shell.surfaces.get(&id).is_none() {
                continue;
            }
            return shell.view(id);
        }
        "".into()
    }
}
