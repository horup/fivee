use bevy::prelude::Component;

#[derive(Component, Default, Debug)]
pub struct AI {
    pub timeout_timer:f32
}