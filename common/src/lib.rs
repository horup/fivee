mod token;
pub use token::*;
mod grid_cell;
pub use grid_cell::*;
mod grid;
pub use grid::*;

use bevy::prelude::*;
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(0));
    }
}