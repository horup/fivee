use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct WorldCursor {
    pub pos:Vec3,
    pub grid_pos:IVec2
}