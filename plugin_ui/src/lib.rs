use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
};
use common::{CommonAssets, Round, RoundCommand, Selection, Token, Grid};

mod components;
pub use components::*;
mod systems;
pub use systems::*;
mod events;
pub use events::*;

fn _update_world_cursor(
    mut commands: Commands,
    mut cursor_moved_events: EventReader<CursorMoved>,
    query_camera: Query<(&GlobalTransform, &Camera)>,
    mut world_cursor: Query<(&mut WorldCursor, &mut Transform)>,
    mouse: Res<Input<MouseButton>>,
    mut ui: ResMut<UI>,
    mut round: ResMut<Round>,
    tokens: Query<(Entity, &Token)>,
    selections: Query<(Entity, &Selection, &Parent)>,
    ca: Res<CommonAssets>,
    grid: Res<Grid>
) {
    let (global_transform_camera, camera) = query_camera.single();
    let (mut world_cursor, mut world_cursor_transform) = world_cursor.single_mut();
    for e in cursor_moved_events.iter() {
        let pos = e.position;
        let ray = camera.viewport_to_world(global_transform_camera, pos);
        if let Some(ray) = ray {
            let n = Vec3::new(0.0, 0.0, 1.0);
            let denom = n.dot(ray.direction);
            if denom.abs() > 0.001 {
                let t = -ray.origin.dot(n) / denom;
                let p = ray.direction * t + ray.origin;
                let grid_pos = p.truncate().as_ivec2();
                world_cursor.grid_pos = grid_pos;
                world_cursor.pos = p;
                world_cursor_transform.translation =
                    grid_pos.as_vec2().extend(0.0) + Vec3::new(0.5, 0.5, 0.0);
            }
        }
    }

    if round.executing() {
        return;
    }

    ui.grid_cursor = world_cursor.grid_pos;
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(selected_entity) = ui.selected_entity {
            if let Ok(_token) = tokens.get(selected_entity) {
                round.push_back_command(RoundCommand::move_to(
                    selected_entity,
                    ui.grid_cursor,
                ))
            } else {
                ui.selected_entity = None;
            }
        } else {
            // no entity selected
            let grid_pos = ui.grid_cursor;
            for (e, token) in tokens.iter() {
                if token.grid_pos == grid_pos {
                    ui.selected_entity = Some(e);
                    let selected_e = commands
                        .spawn(PbrBundle {
                            mesh: ca.mesh("selector"),
                            material: ca.material("white"),
                            ..Default::default()
                        })
                        .insert(Selection::default())
                        .id();
                    commands.entity(e).add_child(selected_e);
                }
            }
        }
    }

    if mouse.just_pressed(MouseButton::Right) {
        ui.selected_entity = None;
    }

    for (token_entity, _) in tokens.iter() {
        for (selection_entity, _, parent) in selections.iter() {
            if parent.get() == token_entity && ui.selected_entity != Some(token_entity) {
                commands.entity(selection_entity).despawn_recursive();
            }
        }
    }
}

pub struct PluginUI;
impl Plugin for PluginUI {
    fn build(&self, app: &mut App) {
        app.insert_resource(UI::default());
        app.add_systems(Startup, startup_system);
        app.add_systems(PreUpdate, (camera_system, world_cursor_position_system));
        //app.add_systems(Update, camera_system);
        app.add_systems(PostUpdate, debug_system);
    }
}
