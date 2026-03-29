use iced_layershell::{
    build_pattern::daemon,
    settings::{LayerShellSettings, StartMode},
};
use wayland_client::Connection;

mod data;
mod dbus;
mod messages;
mod session;
mod shell;
mod surfaces;
mod widgets;
use session::Session;

fn main() {
    let conn = std::sync::Arc::new(Connection::connect_to_env().unwrap());
    let settings_conn = (*conn).clone();
    let session_conn = conn.clone();
    let _ = daemon(
        move || Session::new(session_conn.clone()),
        "Cristal_Shell",
        Session::message,
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
    .theme(Session::theme)
    .subscription(Session::subscription)
    .run();
}
