use bevy::prelude::*;

mod systems;

pub struct PluginGame;
impl Plugin for PluginGame {
    fn build(&self, app: &mut App) {
        systems::add_systems(app);
    }
}
