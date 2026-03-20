use iced::theme::Theme;
use iced_layershell::{
    build_pattern::daemon,
    settings::{LayerShellSettings, StartMode},
};
use wayland_client::Connection;

mod dbus;
mod messages;
mod session;
mod shell;
mod surfaces;
mod wayland;
mod widgets;
use session::Session;

fn main() {
    let conn = std::sync::Arc::new(Connection::connect_to_env().unwrap());
    let settings_conn = (*conn).clone();
    let session_conn = conn.clone();
    let _ = daemon(
        move || Session::new(session_conn.clone()),
        "Cristal_Shell",
        Session::update,
        Session::view,
    )
    .settings(iced_layershell::Settings {
        id: None,
        layer_settings: LayerShellSettings {
            start_mode: StartMode::Background,
            ..Default::default()
        },
        with_connection: Some(settings_conn),
        ..Default::default()
    })
    .style(Session::style)
    .theme(Theme::CatppuccinFrappe)
    .subscription(Session::subscription)
    .run();
}
