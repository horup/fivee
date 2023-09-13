use bevy::prelude::{KeyCode, Resource};

#[derive(Resource)]
pub struct Settings {
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub pan_left: KeyCode,
    pub pan_right: KeyCode,
    pub pan_up: KeyCode,
    pub pan_down: KeyCode,
    pub rotate_left: KeyCode,
    pub rotate_right: KeyCode,
    pub rotate_speed:f32
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rotate_speed:2.0,
            pan_speed: 10.0,
            zoom_speed: 1.0,
            pan_left: KeyCode::A,
            pan_right: KeyCode::D,
            pan_up: KeyCode::W,
            pan_down: KeyCode::S,
            rotate_left: KeyCode::Q,
            rotate_right: KeyCode::E,
        }
    }
}
