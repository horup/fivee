/*


use core::{
    glam::{IVec2, Vec2},
    registry::EntityId,
    Facade,
};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct ReachableCell {
    pub to: IVec2,
    pub cost_ft: i16,
    pub from :IVec2
}

fn move_from(
    f: &Facade,
    pos: IVec2,
    h: &mut HashMap<IVec2, ReachableCell>,
    movement_total_ft: i16,
    movement_cost_ft: i16,
) {
    let ds = [
        IVec2::new(-1, -1),
        IVec2::new(0, -1),
        IVec2::new(1, -1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
        IVec2::new(-1, 1),
        IVec2::new(0, 1),
        IVec2::new(1, 1),
    ];
    for d in ds {
        let new_pos = pos + d;
        if let Some(cell) = f.map.get(new_pos.x as usize, new_pos.y as usize) {
            if cell.blocked {
                continue;
            }
        }

        let movement_cost_ft = movement_cost_ft + 5;
        if movement_cost_ft > movement_total_ft {
            continue;
        }

        if let Some(e) = h.get_mut(&new_pos) {
            if e.cost_ft > movement_cost_ft {
                e.cost_ft = movement_cost_ft;
                e.from = pos;
                move_from(f, new_pos, h, movement_total_ft, movement_cost_ft);
            }
        } else {
            h.insert(
                new_pos,
                ReachableCell {
                    from:pos,
                    to: new_pos,
                    cost_ft: movement_cost_ft,
                },
            );
            move_from(f, new_pos, h, movement_total_ft, movement_cost_ft);
        }
        //move_from(f, new_pos, h, movement_total_ft, movement_cost_ft);
    }
}

pub fn get_reachable_cells(f: &Facade, entity: EntityId) -> HashMap<IVec2, ReachableCell> {
    let mut vec = HashMap::new();
    if let Some(token) = f.tokens.get(entity) {
        move_from(f, token.pos, &mut vec, token.movement, 0);
        vec.remove(&token.pos);
    }
    vec
}
*/

use bevy::{prelude::*, utils::HashMap};
use common::{Grid, Token};


#[derive(Clone, Copy)]
pub struct ReachableCell {
    pub to: IVec2,
    pub cost_ft: i16,
    pub from :IVec2
}


fn move_from(pos:IVec2, map:&mut HashMap<IVec2, ReachableCell>, grid:&Grid, movement_total_ft: i16, movement_cost_ft: i16) {
    let ds = [
        IVec2::new(-1, -1),
        IVec2::new(0, -1),
        IVec2::new(1, -1),
        IVec2::new(-1, 0),
        IVec2::new(1, 0),
        IVec2::new(-1, 1),
        IVec2::new(0, 1),
        IVec2::new(1, 1),
    ];
    for d in ds {
        let new_pos = pos + d;
        if let Some(cell) = grid.get(IVec2::new(new_pos.x, new_pos.y)) {
            if cell.blocked {
                continue;
            }
        }

        let movement_cost_ft = movement_cost_ft + 5;
        if movement_cost_ft > movement_total_ft {
            continue;
        }

        if let Some(e) = map.get_mut(&new_pos) {
            if e.cost_ft > movement_cost_ft {
                e.cost_ft = movement_cost_ft;
                e.from = pos;
                move_from(new_pos, map, grid, movement_total_ft, movement_cost_ft);
            }
        } else {
            map.insert(
                new_pos,
                ReachableCell {
                    from:pos,
                    to: new_pos,
                    cost_ft: movement_cost_ft,
                },
            );
            move_from(new_pos, map, grid, movement_total_ft, movement_cost_ft);
        }
    }
}

pub fn get_reachable_cells(token:&Token, grid:&Grid) -> HashMap<IVec2, ReachableCell> {
    let mut map = HashMap::new();
    let start_pos = token.grid_pos;
    move_from(token.grid_pos, &mut map, grid, 30, 0);
    map.remove(&start_pos);
    map
}