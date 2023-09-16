use bevy::prelude::*;
use crate::ShortLived;

fn kill_system(mut commands:Commands, q:Query<(Entity, &ShortLived)>) {
    for (e, s) in q.iter() {
        if s.despawn {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn age_system(_commands:Commands, mut q:Query<&mut ShortLived>) {
    for mut s in q.iter_mut() {
        s.despawn = true;
    }
}

pub fn build(app:&mut App) {
    app.add_systems(Last, (kill_system, age_system).chain());
}