use bevy::prelude::*;

#[derive(Component)]
pub struct UIDebugFPS;


#[derive(Resource, Default)]
pub struct UI {
    pub selected_entity:Option<Entity>,
    pub grid_cursor:IVec2
}