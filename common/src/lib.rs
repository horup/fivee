mod token;
pub use token::*;
mod grid_cell;
pub use grid_cell::*;
mod grid;
pub use grid::*;
mod ui;
pub use ui::*;
mod world_cursor;
pub use world_cursor::*;
mod assets;
pub use assets::*;

use bevy::prelude::*;
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(0));
        app.insert_resource(WorldCursor::default());
        app.insert_resource(CommonAssets::default());
    }
}