use bevy::prelude::*;
use common::{CommonAssets, Token, Round, RoundCommand, Grid};
mod systems;

pub struct PluginGame;
impl Plugin for PluginGame {
    fn build(&self, app: &mut App) {
        systems::add_systems(app);
    }
}
