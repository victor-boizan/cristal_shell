use crate::messages::Message;
use crate::wayland::workspaces::Workspace;
use crate::wayland::workspaces::WorkspaceState;
use crate::widgets::button::Button;
use iced::{
    color,
    widget::{button, container, container::Style, Column},
    Element,
};
pub struct WorkspacesList {
    workspaces: Vec<Workspace>,
    buttons: Vec<Button>,
}

impl WorkspacesList {
    pub fn new(workspaces: Vec<Workspace>) -> Self {
        let mut buttons: Vec<Button> = Vec::new();

        for (id, ws) in workspaces.clone().into_iter().enumerate() {
            buttons.push(Button::new(
                (id + 1).to_string(),
                ws.state == WorkspaceState::Active,
            ));
        }
        Self {
            workspaces,
            buttons,
        }
    }
    pub fn update(&mut self, workspaces: Vec<Workspace>) {
        self.workspaces = workspaces.clone();
        self.buttons = Vec::new();
        for (id, ws) in workspaces.clone().into_iter().enumerate() {
            self.buttons.push(Button::new(
                (id + 1).to_string(),
                ws.state == WorkspaceState::Active,
            ));
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let mut col = Column::new();
        for button in &self.buttons {
            col = col.push(button.view());
        }
        container(col)
            .style(|theme| Style {
                text_color: Some(theme.clone().palette().text),
                background: Some(iced::Background::Color(
                    theme.clone().extended_palette().background.weak.color,
                )),
                border: iced::Border {
                    color: color!(0x000000),
                    width: 0.0,
                    radius: iced::border::Radius::new(10),
                },
                shadow: iced::Shadow::default(),
                snap: true, //IDK WTF is this.
            })
            .into()
    }
}
