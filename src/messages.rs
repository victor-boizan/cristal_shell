use crate::wayland::WaylandState;
use iced_layershell::to_layer_message;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
    Update(Update),
    Action(Action),
}
#[derive(Debug, Clone)]
pub enum Update {
    WaylandInit(WaylandState),
    WaylandUpdate(WaylandState),
    ToggleDashBoard,
    Tick,
}
#[derive(Debug, Clone)]
pub enum Action {
    //Tick,
}
