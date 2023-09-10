use bevy::prelude::*;

#[derive(Default, Component)]
pub struct ShortLived {
    pub despawn:bool
}
