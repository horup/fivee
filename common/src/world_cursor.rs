use bevy::prelude::*;

#[derive(Default, Component)]
pub struct WorldCursor {
    pub pos:Vec3,
    pub grid_pos:IVec2
}