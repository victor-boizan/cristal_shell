use crate::messages::Message;
use crate::wayland::workspaces::Workspace;
use crate::widgets::button::Button;
use iced::{widget::button, widget::container, widget::Column, Element};
pub struct WorkspacesList {
    workspaces: Vec<Workspace>,
    buttons: Vec<Button>,
}

impl WorkspacesList {
    pub fn new(workspaces: Vec<Workspace>) -> Self {
        let mut buttons: Vec<Button> = Vec::new();
        for ws in workspaces.clone() {
            buttons.push(Button::new("~".to_string()));
        }

        Self {
            workspaces,
            buttons,
        }
    }
    pub fn update(&mut self, workspaces: Vec<Workspace>) {
        self.workspaces = workspaces.clone();
        self.buttons = Vec::new();
        for ws in workspaces.clone() {
            self.buttons.push(Button::new("~".to_string()));
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let mut col = Column::new();
        for button in &self.buttons {
            col = col.push(button.view());
        }
        container(col).into()
    }
}
