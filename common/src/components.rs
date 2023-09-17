use bevy::prelude::*;
use glam::*;

#[derive(Component, Default)]
pub struct Token {
    pub color:Color,
    pub image:String,
    pub statblock:String,
    pub grid_pos:IVec2, 
    pub name:String,
    pub player:Option<Entity>,
    pub movement_ft:f32,
}

impl Token {
    pub fn pos(grid_pos:IVec2) -> Vec3 {
        Vec3::new(grid_pos.x as f32 + 0.5,  grid_pos.y as f32 + 0.5, 0.0)
    }
}

#[derive(Default, Component)]
pub struct ShortLived {
    pub despawn:bool
}

#[derive(Component)]
pub struct Selection {
    pub entity:Entity
}


#[derive(Component, Default)]
pub struct Player {
    pub name:String
}