use iced_layershell::{
    settings::{LayerShellSettings, StartMode},
    build_pattern::daemon,
};
use iced::theme::Theme;
use wayland_client::Connection;

mod wayland;
mod messages;
mod shell;
mod surfaces;
mod session;
use session::Session;

fn main() {
    let conn = std::sync::Arc::new(Connection::connect_to_env().unwrap());
    let settings_conn = (*conn).clone();
    let session_conn = conn.clone();
    let _ = daemon(
        move || Session::new(session_conn.clone()),
        "Cristal_Shell",
        Session::update,
        Session::view
    ).settings(iced_layershell::Settings{
        id: None,
        layer_settings: LayerShellSettings {
            start_mode: StartMode::Background,
            ..Default::default()
        },
	with_connection: Some(settings_conn),
        ..Default::default()
    }).style(Session::style)
    .theme(Theme::CatppuccinFrappe)
    .subscription(Session::subscription)
    .run();
}
