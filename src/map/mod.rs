pub mod test;

use bevy::prelude::Plugin;
use test::MapTestPlugin;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(MapTestPlugin);
    }
}
