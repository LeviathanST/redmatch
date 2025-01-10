mod crosshair;

use bevy::{app::Startup, prelude::Plugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, crosshair::spawn);
    }
}
