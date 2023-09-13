use bevy::prelude::{Component, Color, Entity};
use glam::*;

#[derive(Component)]
pub struct Token {
    pub color:Color,
    pub grid_pos:IVec2, 
    pub name:String,
    pub player:Option<Entity>
}

impl Token {
    pub fn pos(grid_pos:IVec2) -> Vec3 {
        Vec3::new(grid_pos.x as f32 + 0.5,  grid_pos.y as f32 + 0.5, 0.0)
    }
}