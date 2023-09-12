use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Settings {

}

impl Default for Settings {
    fn default() -> Self {
        Self {  }
    }
}