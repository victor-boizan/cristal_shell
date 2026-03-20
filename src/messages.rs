use crate::wayland::WaylandState;
use iced_layershell::to_layer_message;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
    Update(Update),
}
#[derive(Debug, Clone)]
pub enum Update {
    WaylandInit(WaylandState),
    WaylandUpdate(WaylandState),
    Tick,
    Toggle,
}
#[derive(Debug, Clone)]
pub enum Action {
    Tick,
}
