use bevy::prelude::Plugin;

use crate::{map::MapPlugin, player::PlayerPlugin, ui::UiPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((MapPlugin, PlayerPlugin, UiPlugin));
    }
}
