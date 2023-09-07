use bevy::prelude::{Component, Color};
use glam::*;

#[derive(Component)]
pub struct Token {
    pub color:Color,
    pub grid_pos:IVec2, 
}

impl Token {
    pub fn pos(&self) -> Vec3 {
        Vec3::new(self.grid_pos.x as f32 + 0.5,  self.grid_pos.y as f32 + 0.5, 0.0)
    }
}