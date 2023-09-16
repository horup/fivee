pub mod math;
mod components;
pub use components::*;
mod events;
pub use events::*;
mod resources;
pub use resources::*;
use bevy::prelude::*;
mod systems;
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        events::add_events(app);
        resources::insert_resources(app);
        systems::add_systems(app);
    }
}
