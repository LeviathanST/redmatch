use bevy::prelude::{IntoSystemConfigs, Plugin};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use crate::{map::MapPlugin, player::PlayerPlugin, ui::UiPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins((MapPlugin, PlayerPlugin, UiPlugin));
    }
}
