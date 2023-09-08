use array2d::Array2D;
use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct GridCell {
    pub blocked:bool,
    pub walkable:bool,
    pub entity:Option<Entity>
}

#[derive(Resource)]
pub struct Grid {
    size: usize,
    cells: Array2D<GridCell>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cells: Array2D::filled_with(GridCell::default(), size, size),
        }
    }

    pub fn get_mut(&mut self, i: IVec2) -> Option<&mut GridCell> {
        if let Some(cell) = self.cells.get_mut(i.x as usize, i.y as usize) {
            return Some(cell);
        }

        return None;
    }

    pub fn get(&self, i: IVec2) -> Option<&GridCell> {
        if let Some(cell) = self.cells.get(i.x as usize, i.y as usize) {
            return Some(cell);
        }

        return None;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_walkable(&self, i: IVec2) -> bool {
        if let Some(cell) = self.get(i) {
            return cell.walkable;
        }
        false
    }

    pub fn is_blocked(&self, i: IVec2) -> bool {
        if let Some(cell) = self.get(i) {
            return cell.blocked;
        }
        false
    }
}
 