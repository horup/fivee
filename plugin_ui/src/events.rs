use bevy::prelude::{Entity, Event, IVec2};

#[derive(Event)]
pub struct GridCursorEvent {
    pub old_pos: IVec2,
    pub grid_pos: IVec2,
    pub left_just_pressed: bool,
    pub right_just_pressed: bool,
}