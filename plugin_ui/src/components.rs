use bevy::prelude::*;

#[derive(Component)]
pub struct UIDebugFPS;

#[derive(Component)]
pub struct UITurnOwnerName;


#[derive(Resource, Default)]
pub struct UI {
    pub player:Option<Entity>,
    pub selected_token:Option<Entity>,
    pub grid_cursor:IVec2
}

#[derive(Default, Component)]
pub struct WorldCursor {
    pub pos:Vec3,
    pub grid_pos:IVec2
}

#[derive(Default, Component)]
pub struct HighlightedCell {
    pub grid_pos:IVec2,
}

#[derive(Default, Component)]
pub struct Waypoint {
    pub grid_pos:IVec2,
}
