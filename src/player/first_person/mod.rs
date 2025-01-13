use bevy::{
    app::{Plugin, Update},
    prelude::{in_state, IntoSystemConfigs},
};

use super::ViewMode;

mod camera;

pub struct FirstPersonPlugin;

impl Plugin for FirstPersonPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            camera::update.run_if(in_state(ViewMode::FirstPerson)),
        );
    }
}
