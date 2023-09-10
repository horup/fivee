use bevy::prelude::{IVec2, Event, Entity};

#[derive(Event)]
pub struct GridCursorEvent {
    pub old_pos:IVec2,
    pub grid_pos:IVec2,
    pub left_just_pressed:bool,
}


#[derive(Event)]
pub struct TokenSelectedEvent {
    pub selected:Option<Entity>,
    pub deselected:Option<Entity>
}