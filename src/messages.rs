use iced_layershell::to_layer_message;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
    Update(Update),
}
#[derive(Debug, Clone)]
pub enum Update {
    Tick,
}
#[derive(Debug, Clone)]
pub enum Action {
    Tick,
}
