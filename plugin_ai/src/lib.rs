use bevy::prelude::Plugin;

pub mod components;
pub mod systems;

pub struct PluginAI;

impl Plugin for PluginAI {
    fn build(&self, app: &mut bevy::prelude::App) {
        systems::add_systems(app);
    }
}