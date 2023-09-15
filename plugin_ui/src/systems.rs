use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
};
use common::{
    CommonAssets, GameEvent, Grid, Player, Round, RoundCommand, Selection, Settings, ShortLived,
    Token,
};

use crate::{
    GridCursorEvent, HighlightedCell, UIDebugFPS, UITurnOwnerName, Waypoint, WorldCursor, UI,
};

fn startup_system(mut commands: Commands, common_assets: ResMut<CommonAssets>) {
    // spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 0.0, 8.0).looking_at(Vec3::new(5.0, 8.0, 0.0), Vec3::Y),
        ..default()
    });

    // spawn debug
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

    // spawn turn owner name
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn(
                    TextBundle::from_section(
                        "---",
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        top: Val::Px(5.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_text_alignment(TextAlignment::Center),
                )
                .insert(UITurnOwnerName);
        });
}

fn camera_system(
    keys: Res<Input<KeyCode>>,
    mut camera: Query<(&mut Camera3d, &mut Transform)>,
    time: Res<Time>,
    mut mouse_wheel: EventReader<MouseWheel>,
    settings: Res<Settings>,
) {
    let (_amera, mut transform) = camera.single_mut();
    let dt = time.delta_seconds();

    // rotate camera
    let mut r = 0.0;
    if keys.pressed(settings.rotate_left) {
        r -= settings.rotate_speed;
    }
    if keys.pressed(settings.rotate_right) {
        r += settings.rotate_speed;
    }
    let forward = transform.forward();
    let ray = Ray {
        origin: transform.translation,
        direction: forward,
    };

    let mut look_at = ray_plane_intersection(ray).unwrap();
    look_at.z = 0.0;
    let mut v = look_at - transform.translation;
    let vz = v.z;
    v.z = 0.0;
    let mut v = Quat::from_rotation_z(r * dt) * v;
    v.z = vz;
    transform.translation = look_at - v;
    transform.look_at(look_at, Vec3::Z);

    // zoom camera
    let mut zoom_delta = 0.0;
    for ev in mouse_wheel.iter() {
        zoom_delta += ev.y * settings.zoom_speed;
    }

    let v = transform.translation + transform.forward() * zoom_delta;
    let max_zoom = 5.0;
    let min_zoom = 100.0;
    if v.z > max_zoom && v.z < min_zoom {
        transform.translation = v;
    }

    // pan camera
    let zoom_factor = v.z / 6.0;
    let mut v = Vec2::new(0.0, 0.0);
    let forward = transform.forward().truncate().normalize_or_zero();
    let side = Vec2::new(-forward.y, forward.x);
    if keys.pressed(settings.pan_left) {
        v += side;
    }
    if keys.pressed(settings.pan_right) {
        v -= side;
    }
    if keys.pressed(settings.pan_up) {
        v += forward;
    }
    if keys.pressed(settings.pan_down) {
        v -= forward;
    }

    let v = v * settings.pan_speed * zoom_factor * dt;
    transform.translation += v.extend(0.0);
}

fn update_debug_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<UIDebugFPS>>,
) {
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
            right_just_pressed,
        });
    }
}

fn ray_plane_intersection(ray: Ray) -> Option<Vec3> {
    let n = Vec3::new(0.0, 0.0, 1.0);
    let denom = n.dot(ray.direction);
    if denom.abs() > 0.001 {
        let t = -ray.origin.dot(n) / denom;
        let p = ray.direction * t + ray.origin;
        return Some(p);
    }

    return None;
}

fn grid_cursor_system(
    mut ui: ResMut<UI>,
    mut reader: EventReader<GridCursorEvent>,
    tokens: Query<(Entity, &Token)>,
    mut round: ResMut<Round>,
) {
    if round.is_executing() {
        return;
    }
    for ev in reader.iter() {
        let grid_pos = ev.grid_pos;
        if ev.left_just_pressed {
            if let Some(selected_entity) = ui.selected_token {
                round.push_front_command(RoundCommand::move_far(selected_entity, grid_pos))
            }
        }
    }
}

fn token_selected_system(
    mut commands: Commands,
    ca: Res<CommonAssets>,
    mut selections: Query<(&Selection, &mut ShortLived)>,
    ui: Res<UI>,
) {
    if let Some(selected_token) = ui.selected_token {
        let mut found = false;
        for (selection, mut sl) in selections.iter_mut() {
            if selection.entity == selected_token {
                sl.despawn = false;
                found = true;
            }
        }

        if !found {
            let selected_e = commands
                .spawn(PbrBundle {
                    mesh: ca.mesh("selector"),
                    material: ca.material("white"),
                    ..Default::default()
                })
                .insert(Selection {
                    entity: selected_token,
                })
                .insert(ShortLived::default())
                .id();
            commands.entity(selected_token).add_child(selected_e);
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
    round: Res<Round>,
) {
    if round.is_executing() {
        return;
    }
    if let Some(selected_entity) = ui.selected_token {
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
    round: Res<Round>,
) {
    if round.is_executing() {
        return;
    }

    if let Some(selected_entity) = ui.selected_token {
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
                            )
                            .with_scale(Vec3::splat(0.5)),
                            ..Default::default()
                        })
                        .insert(Waypoint { grid_pos: cell.to })
                        .insert(ShortLived::default());
                }
            }
        }
    }
}

fn action_system(ui: Res<UI>, mut round: ResMut<Round>, keys: Res<Input<KeyCode>>) {
    if round.is_executing() {
        return;
    }
    if let Some(entity) = ui.selected_token {
        if keys.just_pressed(KeyCode::Space) {
            round.push_back_command(RoundCommand::give_turn(entity));
        }
    }
}

fn update_turn_owner_name_system(
    round: Res<Round>,
    tokens: Query<&Token>,
    mut turn_owner_name: Query<&mut Text, With<UITurnOwnerName>>,
) {
    let mut turn_owner_name = turn_owner_name.single_mut();
    turn_owner_name.sections[0].value = format!("Round {}", round.round_num);
    if let Some(turn_owner) = round.active_entity {
        if let Ok(turn_owner) = tokens.get(turn_owner) {
            turn_owner_name.sections[0].value = turn_owner.name.clone();
        }
    }
}

fn ensure_player_system(q: Query<Entity, With<Player>>, mut ui: ResMut<UI>) {
    let e = q.single();
    ui.player = Some(e);
}

fn select_my_active_token_system(round: Res<Round>, mut ui: ResMut<UI>, tokens: Query<&Token>) {
    if round.is_executing() {
        return;
    }

    ui.selected_token = None;
    let Some(active_token) = round.active_entity else {
        return;
    };
    let Ok(token) = tokens.get(active_token) else {
        return;
    };
    if token.player == ui.player {
        ui.selected_token = Some(active_token);
    }
}

pub fn pan_to_active_entity_system(
    mut reader: EventReader<GameEvent>,
    mut transforms: Query<&mut Transform>,
    mut cameras: Query<(Entity, &Camera)>,
) {
    let mut pan_to = None;
    for ev in reader.iter() {
        match ev {
            GameEvent::IsNowActive { entity } => {
                if let Ok(t) = transforms.get(*entity) {
                    pan_to = Some(t.translation);
                }
            }
            _ => {}
        }
    }

    let Some(pan_to) = pan_to else { return };
    let camera = cameras.single();
    let mut camera_transform = transforms.get_mut(camera.0).unwrap();
    camera_transform.translation.x = pan_to.x;
    camera_transform.translation.y = pan_to.y;
}

pub fn add_systems(app: &mut App) {
    app.add_systems(Startup, startup_system);
    app.add_systems(
        Update,
        (
            ensure_player_system,
            select_my_active_token_system,
            camera_system,
            pan_to_active_entity_system,
            cursor_changed_system,
            grid_cursor_system,
            token_selected_system,
            highlight_system,
            waypoint_system,
            action_system,
            update_turn_owner_name_system,
        )
            .chain(),
    );
    app.add_systems(PostUpdate, update_debug_system);
}
