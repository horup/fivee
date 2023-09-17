pub mod math;
mod components;
pub use components::*;
mod events;
pub use events::*;
mod resources;
pub use resources::*;
use bevy::prelude::*;
mod systems;
mod assets;
pub use assets::*;
mod bundles;
pub use bundles::*;
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        events::build(app);
        resources::build(app);
        systems::build(app);
        assets::build(app);
    }
}
