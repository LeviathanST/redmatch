mod game;
mod map;
mod player;
mod ui;

use bevy::{
    app::{App, Startup, Update},
    diagnostic::LogDiagnosticsPlugin,
    prelude::{Res, ResMut},
    DefaultPlugins,
};
use bevy_framepace::{debug::DiagnosticsPlugin, FramepacePlugin, FramepaceSettings, Limiter};
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FramepacePlugin,
            DiagnosticsPlugin,
        ))
        .add_systems(Startup, setup_screen)
        .add_systems(Update, log)
        .add_plugins(GamePlugin)
        .run();
}

/// TODO: Setup debug logs
fn setup_screen(mut frame_setting: ResMut<FramepaceSettings>) {
    frame_setting.limiter = Limiter::from_framerate(60.);
}
fn log(frame_setting: Res<FramepaceSettings>) {
    println!("{:?}", frame_setting.limiter);
}
