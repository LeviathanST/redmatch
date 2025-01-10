mod game;
mod map;
mod player;
mod ui;

use bevy::{app::App, DefaultPlugins};
use game::GamePlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
