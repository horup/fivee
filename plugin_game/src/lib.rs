use bevy::prelude::*;
use common::{CommonAssets, Token, Round, RoundCommand, Grid};
mod systems;
pub use systems::*;

pub struct PluginGame;
impl Plugin for PluginGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, update_round_system);
        app.add_systems(PostUpdate, on_spawn_token_system);
    }
}
