mod camera;
use bevy::{
    app::Update,
    prelude::{in_state, IntoSystemConfigs, Plugin},
};

use super::ViewMode;

pub struct ThirdPersonPlugin;

impl Plugin for ThirdPersonPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            camera::update.run_if(in_state(ViewMode::ThirdPerson)),
        );
    }
}
