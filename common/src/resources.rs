use bevy::{prelude::*, utils::HashMap};
use glam::IVec2;
use std::collections::VecDeque;

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
    pub rotate_speed: f32,
    pub auto_pan_speed: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rotate_speed: 2.0,
            pan_speed: 10.0,
            auto_pan_speed: 5.0,
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

pub enum Variant {
    Nop,
    MoveTo { who: Entity, to: IVec2 },
    MoveFar { who: Entity, to: IVec2 },
    GiveTurn { who: Entity },
    EndRound {},
}

impl Default for Variant {
    fn default() -> Self {
        Self::Nop
    }
}

#[derive(Default)]
pub struct RoundCommand {
    pub timer: f32,
    pub timer_elapsed_sec: f32,
    pub parallel: bool,
    pub variant: Variant,
}

impl RoundCommand {
    pub fn nop() -> Self {
        Self {
            timer: 0.5,
            variant: Variant::Nop,
            ..Default::default()
        }
    }

    pub fn move_to(who: Entity, to: IVec2) -> Self {
        Self {
            timer: 0.2,
            variant: Variant::MoveTo { who, to },
            ..Default::default()
        }
    }

    pub fn move_far(who: Entity, to: IVec2) -> Self {
        Self {
            variant: Variant::MoveFar { who, to },
            ..Default::default()
        }
    }

    pub fn give_turn(who: Entity) -> Self {
        Self {
            variant: Variant::GiveTurn { who },
            timer: 0.25,
            ..Default::default()
        }
    }

    pub fn end_round() -> Self {
        Self {
            timer: 0.25,
            variant: Variant::EndRound {},
            ..Default::default()
        }
    }
    pub fn alpha(&self) -> f32 {
        if self.timer == 0.0 {
            return 1.0;
        }
        let a = self.timer_elapsed_sec / self.timer;
        if a < 0.0 {
            return 0.0;
        } else if a > 1.0 {
            return a;
        }

        a
    }
}

#[derive(Resource, Default)]
pub struct Round {
    commands: VecDeque<RoundCommand>,
    pub active_entity: Option<Entity>,
    pub initiative_order: Vec<Entity>,
    pub has_taken_turn: HashMap<Entity, ()>,
    pub round_num: u64,
}

impl Round {
    pub fn front_mut(&mut self) -> Option<&mut RoundCommand> {
        self.commands.front_mut()
    }

    pub fn push_front(&mut self, command: RoundCommand) {
        self.commands.push_front(command);
    }

    pub fn push_back(&mut self, command: RoundCommand) {
        self.commands.push_back(command);
    }

    pub fn pop_front(&mut self) -> Option<RoundCommand> {
        self.commands.pop_front()
    }

    pub fn is_executing(&self) -> bool {
        !self.commands.is_empty()
    }
}

use array2d::Array2D;

#[derive(Default, Clone)]
pub struct GridCell {
    pub blocked: bool,
    pub walkable: bool,
    pub entity: Option<Entity>,
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

        None
    }

    pub fn get(&self, i: IVec2) -> Option<&GridCell> {
        if let Some(cell) = self.cells.get(i.x as usize, i.y as usize) {
            return Some(cell);
        }

        None
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

#[derive(Resource, Default)]
pub struct CommonAssets {
    fonts: HashMap<&'static str, Handle<Font>>,
    materials: HashMap<&'static str, Handle<StandardMaterial>>,
    meshes: HashMap<&'static str, Handle<Mesh>>,
    images: HashMap<&'static str, Handle<Image>>,
}

impl CommonAssets {
    pub fn font(&self, id: &str) -> Handle<Font> {
        self.fonts.get(id).unwrap_or_else(|| panic!("{} font not found!", id)).clone()
    }
    pub fn font_insert(&mut self, id:&'static str, handle:Handle<Font>) {
        self.fonts.insert(id, handle);
    }

    pub fn material(&self, id: &str) -> Handle<StandardMaterial> {
        self.materials.get(id).unwrap_or_else(|| panic!("{} material not found!", id)).clone()
    }
    pub fn material_insert(&mut self, id:&'static str, handle:Handle<StandardMaterial>) {
        self.materials.insert(id, handle);
    }

    pub fn mesh(&self, id: &str) -> Handle<Mesh> {
        self.meshes.get(id).unwrap_or_else(|| panic!("{} mesh not found!", id)).clone()
    }
    pub fn mesh_insert(&mut self, id:&'static str, handle:Handle<Mesh>) {
        self.meshes.insert(id, handle);
    }

    pub fn image(&self, id: &str) -> Handle<Image> {
        self.images.get(id).unwrap_or_else(|| panic!("{} image not found!", id)).clone()
    }
    pub fn image_insert(&mut self, id:&'static str, handle:Handle<Image>) {
        self.images.insert(id, handle);
    }
}


pub fn build(app: &mut App) {
    app.insert_resource(Grid::new(0));
    app.insert_resource(CommonAssets::default());
    app.insert_resource(Round::default());
    app.insert_resource(Settings::default());
}
