use bevy::prelude::*;

#[derive(Component)]
pub struct UIDebugFPS;


#[derive(Resource, Default)]
pub struct UI {
    pub selected_entity:Option<Entity>,
    pub grid_cursor:IVec2
}

#[derive(Default, Component)]
pub struct WorldCursor {
    pub pos:Vec3,
    pub grid_pos:IVec2
}