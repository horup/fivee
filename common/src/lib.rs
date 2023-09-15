mod token;
pub use token::*;
mod grid;
pub use grid::*;
mod assets;
pub use assets::*;
mod round;
pub use round::*;
pub mod math;
mod short_lived;
pub use short_lived::*;
mod settings;
pub use settings::*;
mod player;
pub use player::*;
mod events;
pub use events::*;

mod selection;
pub use selection::*;

use bevy::prelude::*;
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        events::add_events(app);
        app.insert_resource(Grid::new(0));
        app.insert_resource(CommonAssets::default());
        app.insert_resource(Round::default());
        app.insert_resource(Settings::default());
        app.add_systems(Last, (kill_system, age_system).chain());
    }
}

fn kill_system(mut commands:Commands, q:Query<(Entity, &ShortLived)>) {
    for (e, s) in q.iter() {
        if s.despawn {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn age_system(mut commands:Commands, mut q:Query<&mut ShortLived>) {
    for mut s in q.iter_mut() {
        s.despawn = true;
    }
}

