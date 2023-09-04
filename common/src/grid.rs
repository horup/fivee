use array2d::Array2D;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    size: usize,
    cells: Array2D<Option<Entity>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cells: Array2D::filled_with(None, 0, 0),
        }
    }

    pub fn set(&mut self, i: IVec2, entity: Option<Entity>) {
        if let Some(cell) = self.cells.get_mut(i.x as usize, i.y as usize) {
            *cell = entity;
        }
    }

    pub fn get(&mut self, i: IVec2) -> Option<Entity> {
        if let Some(cell) = self.cells.get(i.x as usize, i.y as usize) {
            return cell.clone();
        }
        None
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
 