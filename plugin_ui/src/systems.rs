use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
};
use common::{CommonAssets, Grid, Selection, ShortLived, Token, Round, RoundCommand};

use crate::{
    GridCursorEvent, HighlightedCell, TokenSelectedEvent, UIDebugFPS, Waypoint, WorldCursor, UI,
};

fn startup_system(mut commands: Commands, common_assets: ResMut<CommonAssets>) {
    // spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 0.0, 8.0).looking_at(Vec3::new(5.0, 8.0, 0.0), Vec3::Y),
        ..default()
    });

    let font = common_assets.font("default");
    commands
        .spawn(
            TextBundle::from_section(
                "---",
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::RED,
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            }),
        )
        .insert(UIDebugFPS);

    // spawn world cusor
    commands
        .spawn(PbrBundle {
            mesh: common_assets.mesh("selector"),
            material: common_assets.material("white"),
            ..Default::default()
        })
        .insert(WorldCursor::default());
}

fn camera_system(
    keys: Res<Input<KeyCode>>,
    mut camera: Query<(&mut Camera3d, &mut Transform)>,
    time: Res<Time>,
    mut mouse_wheel: EventReader<MouseWheel>,
) {
    let (_camera, mut transform) = camera.single_mut();
    let mut v = Vec2::new(0.0, 0.0);
    if keys.pressed(KeyCode::A) {
        v.x -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        v.x += 1.0;
    }
    if keys.pressed(KeyCode::W) {
        v.y += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        v.y -= 1.0;
    }

    let v = v.normalize_or_zero();
    let dt = time.delta_seconds();
    let speed = 10.0;
    let v = v * speed * dt;
    let mut v = v.extend(0.0);
    for ev in mouse_wheel.iter() {
        let sy = ev.y;
        v.z -= sy;
    }

    transform.translation += v;

    let min_z = 5.0;
    if transform.translation.z < 5.0 {
        transform.translation.z = min_z;
    }
}

fn debug_system(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<UIDebugFPS>>) {
    for mut text in &mut query {
        if let Some(fps_diagnostics) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            text.sections[0].value =
                format!("{}", fps_diagnostics.smoothed().unwrap_or_default() as i32);
        }
    }
}

fn cursor_changed_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    query_camera: Query<(&GlobalTransform, &Camera)>,
    mut world_cursor: Query<(&mut WorldCursor, &mut Transform)>,
    mut ui: ResMut<UI>,
    mut writer: EventWriter<GridCursorEvent>,
    buttons: Res<Input<MouseButton>>,
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

    let mut fire = false;
    let old_pos = ui.grid_cursor;
    let pos = world_cursor.grid_pos;
    let left_just_pressed = buttons.just_pressed(MouseButton::Left);
    let right_just_pressed = buttons.just_pressed(MouseButton::Right);
    ui.grid_cursor = world_cursor.grid_pos;
    if pos != old_pos {
        fire = true;
    }
    if left_just_pressed {
        fire = true;
    }
    if right_just_pressed {
        fire = true;
    }

    if fire {
        writer.send(GridCursorEvent {
            old_pos,
            grid_pos: pos,
            left_just_pressed,
            right_just_pressed
        });
    }
}

fn grid_cursor_system(
    mut ui: ResMut<UI>,
    mut reader: EventReader<GridCursorEvent>,
    tokens: Query<(Entity, &Token)>,
    mut writer: EventWriter<TokenSelectedEvent>,
    mut round:ResMut<Round>
) {
    if round.is_executing() {
        return;
    }
    for ev in reader.iter() {
        let grid_pos = ev.grid_pos;
        if ev.left_just_pressed {
            let mut selected: Option<Entity> = None;
            for (e, token) in tokens.iter() {
                if token.grid_pos == grid_pos {
                    selected = Some(e);
                    break;
                }
            }

            if let Some(selected) = selected {
                if Some(selected) != ui.selected_entity {
                    writer.send(TokenSelectedEvent {
                        selected: Some(selected),
                        deselected: ui.selected_entity,
                    });
                    ui.selected_entity = Some(selected);
                }
            } else if ui.selected_entity != None {
                writer.send(TokenSelectedEvent {
                    selected: None,
                    deselected: ui.selected_entity,
                });
                ui.selected_entity = None;
            }
        }
        if ev.right_just_pressed {
            if let Some(selected_entity) = ui.selected_entity {
                dbg!("right pressed");
                round.push_front_command(RoundCommand::move_far(selected_entity, grid_pos))
            }
        }
    }
}

fn entity_selected_system(
    mut commands: Commands,
    mut reader: EventReader<TokenSelectedEvent>,
    ca: Res<CommonAssets>,
    selections: Query<(Entity, &Selection)>,
    ui: Res<UI>,
) {
    for ev in reader.iter() {
        if let Some(e) = ev.selected {
            let selected_e = commands
                .spawn(PbrBundle {
                    mesh: ca.mesh("selector"),
                    material: ca.material("white"),
                    ..Default::default()
                })
                .insert(Selection { entity: e })
                .id();
            commands.entity(e).add_child(selected_e);
        }
    }

    for (selection_entity, selection) in selections.iter() {
        if Some(selection.entity) != ui.selected_entity {
            commands.entity(selection_entity).despawn_recursive();
        }
    }
}

fn highlight_system(
    mut commands: Commands,
    ui: Res<UI>,
    tokens: Query<&Token>,
    grid: Res<Grid>,
    mut highlighted_cells: Query<(Entity, &mut HighlightedCell, &mut ShortLived)>,
    ca: Res<CommonAssets>,
    round:Res<Round>
) {
    if round.is_executing() {
        return;
    }
    if let Some(selected_entity) = ui.selected_entity {
        if let Ok(token) = tokens.get(selected_entity) {
            let reachable_cells = rules::get_reachable_cells(token, &grid);
            for (i, _) in reachable_cells.iter() {
                let i = *i;
                let mut spawn = true;
                for (_, hc, mut sl) in highlighted_cells.iter_mut() {
                    if hc.grid_pos == i {
                        sl.despawn = false;
                        spawn = false;
                    }
                }

                if spawn {
                    commands
                        .spawn(PbrBundle {
                            mesh: ca.mesh("cell"),
                            transform: Transform::from_xyz(
                                i.x as f32 + 0.5,
                                i.y as f32 + 0.5,
                                0.001,
                            ),
                            material: ca.material("highlight_blue"),
                            ..Default::default()
                        })
                        .insert(HighlightedCell { grid_pos: i })
                        .insert(ShortLived::default());
                }
            }
        }
    }
}

fn waypoint_system(
    mut commands: Commands,
    tokens: Query<(&Token)>,
    ui: Res<UI>,
    mut waypoints: Query<(&Waypoint, &mut ShortLived)>,
    grid: Res<Grid>,
    ca: Res<CommonAssets>,
    round:Res<Round>
) {
    if round.is_executing() {
        return;
    }

    if let Some(selected_entity) = ui.selected_entity {
        if let Ok(token) = tokens.get(selected_entity) {
            let path = rules::get_path(token, &grid, ui.grid_cursor);
            for cell in path.iter() {
                let mut spawn = true;
                for (wp, mut sl) in waypoints.iter_mut() {
                    if wp.grid_pos == cell.to {
                        sl.despawn = false;
                        spawn = false;
                        break;
                    }
                } 
                
                if spawn {
                    commands
                        .spawn(PbrBundle {
                            mesh: ca.mesh("token"),
                            material: ca.material("white"),
                            transform: Transform::from_xyz(
                                cell.to.x as f32 + 0.5,
                                cell.to.y as f32 + 0.5,
                                0.001,
                            ).with_scale(Vec3::splat(0.5)),
                            ..Default::default()
                        })
                        .insert(Waypoint { grid_pos: cell.to })
                        .insert(ShortLived::default());
                }
            }
        }
    }
}

pub fn add_systems(app: &mut App) {
    app.add_systems(Startup, startup_system);
    app.add_systems(PreUpdate, (camera_system, cursor_changed_system).chain());
    app.add_systems(
        Update,
        (
            grid_cursor_system,
            entity_selected_system,
            highlight_system,
            waypoint_system,
        ).chain(),
    );
    app.add_systems(PostUpdate, debug_system);
}
