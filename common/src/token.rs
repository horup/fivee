use bevy::prelude::{Component, Color};
use glam::*;

#[derive(Component)]
pub struct Token {
    pub color:Color,
    pub grid_pos:IVec2, 
} 