use iced::{Color, color, theme::Theme, window};
use iced::{Element, Task};
use std::collections::HashMap;
use std::sync::Arc;
use wayland_client::Connection;

use crate::{
    data::wayland::{WaylandState, outputs::Output},
    messages::{Action, Message, Update},
    shell::Shell,
};

pub struct Session {
    shells: HashMap<Output, Shell>, //each output will have its own shell
    theme_mode: ThemeMode,
    conn: Arc<Connection>,
}
#[derive(PartialEq)]
enum ThemeMode {
    Dark,
    Light,
}

impl Session {
    pub fn new(conn: Arc<Connection>) -> (Self, Task<Message>) {
        let shells: HashMap<Output, Shell> = HashMap::new();

        (
            Self {
                shells,
                theme_mode: ThemeMode::Dark,
                conn,
            },
            Task::none(),
        )
    }
    pub fn style(&self, _theme: &iced::Theme) -> iced::theme::Style {
        iced::theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: color!(0xffffff),
        }
    }
    pub fn message(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Update(update) => self.update(update),
            Message::Action(action) => self.action(action),
            Message::Test => {
                println!("test msg");
                Task::none()
            }
            _ => Task::none(),
        }
    }
    fn update(&mut self, update: Update) -> Task<Message> {
        match update {
            Update::WaylandInit(state) => {
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
            Update::ToggleTheme => {
                if self.theme_mode == ThemeMode::Dark {
                    self.theme_mode = ThemeMode::Light;
                } else {
                    self.theme_mode = ThemeMode::Dark;
                }
                Task::none()
            }
            _ => {
                let mut tasks = Vec::new();
                for (_, shell) in &mut self.shells {
                    tasks.push(shell.update(update.clone()));
                }
                Task::batch(tasks)
            }
        }
    }
    fn action(&mut self, action: Action) -> Task<Message> {
        match action {
            Action::ToggleTheme => {
                if self.theme_mode == ThemeMode::Dark {
                    self.theme_mode = ThemeMode::Light;
                } else {
                    self.theme_mode = ThemeMode::Dark;
                }
                Task::none()
            }
            Action::ToggleDashBoard => Task::done(Message::Update(Update::ToggleDashBoard)),
            Action::None => Task::none(),
        }
    }
    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch([
            iced::time::every(std::time::Duration::from_secs(1))
                .map(|_| Message::Update(Update::Tick)),
            WaylandState::subscription(self.conn.clone()).map(|msg| msg),
            crate::dbus::subscription(),
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
    pub fn theme(&self, _id: window::Id) -> Option<Theme> {
        match self.theme_mode {
            ThemeMode::Dark => Some(Theme::CatppuccinFrappe),
            ThemeMode::Light => Some(Theme::CatppuccinLatte),
        }
    }
}
