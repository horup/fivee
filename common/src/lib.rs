mod token;
pub use token::*;
mod grid;
pub use grid::*;
mod ui;
pub use ui::*;
mod world_cursor;
pub use world_cursor::*;
mod assets;
pub use assets::*;
mod round;
pub use round::*;
pub mod math;

mod selection;
pub use selection::*;

use bevy::prelude::*;
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(0));
        app.insert_resource(CommonAssets::default());
        app.insert_resource(Round::default());
        app.insert_resource(UI::default());
    }
}