use bevy::prelude::Component;
use glam::IVec2;

#[derive(Component)]
pub struct GridCell {
    pub pos:IVec2 
}