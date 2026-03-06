use iced_layershell::{
    settings::{LayerShellSettings, StartMode},
    build_pattern::daemon,
};
use iced::theme::Theme;

mod messages;
mod session;
use session::Session;

fn main() {
    let _ = daemon(
        move || Session::new(),
        "Cristal_Shell",
        Session::update,
        Session::view
    ).settings(iced_layershell::Settings{
        id: None,
        layer_settings: LayerShellSettings {
            start_mode: StartMode::Background,
            ..Default::default()
        },
        ..Default::default()
    }).style(Session::style)
    .theme(Theme::CatppuccinFrappe)
    .run();
}
