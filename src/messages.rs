use crate::data::wayland::WaylandState;
use iced_layershell::to_layer_message;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
    Update(Update),
    Action(Action),
    Test, //test message
}
#[derive(Debug, Clone)]
pub enum Update {
    WaylandInit(WaylandState),
    WaylandUpdate(WaylandState),
    ToggleDashBoard,
    ToggleTheme,
    Tick,
}
#[derive(Debug, Clone)]
pub enum Action {
    ToggleTheme,
    ToggleDashBoard,
    None, //do nothing.
}
