use bevy::prelude::*;
use common::{CommonAssets, GameEvent, Grid, Player, Round, RoundCommand, Statblock, Token};
use mapgen::{AreaStartingPosition, BspRooms, MapBuilder, SimpleRooms, XStart, YStart};
use rand::{rngs::StdRng, SeedableRng};

fn startup_system(
    mut commands: Commands,
    sa: Res<CommonAssets>,
    _round: ResMut<Round>,
    asset_server: Res<AssetServer>,
) {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let map_size = 64;
    let mapbuffer = MapBuilder::new(map_size, map_size)
        .with(BspRooms::new())
        .with(SimpleRooms::new())
        .with(mapgen::filter::rooms_corridors_nearest::NearestCorridors::new())
        .with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP))
        .build_with_rng(&mut rng);

    let mut grid = Grid::new(map_size);
    for y in 0..map_size {
        for x in 0..map_size {
            let i = IVec2 {
                x: x as i32,
                y: y as i32,
            };
            let blocked = mapbuffer.is_blocked(x, y);
            let walkable = mapbuffer.is_walkable(x, y);

            let x = x as f32 + 0.5;
            let y = y as f32 + 0.5;

            if blocked {
                commands.spawn(PbrBundle {
                    mesh: sa.mesh("cube"),
                    material: sa.material("brick"),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                });
                commands.spawn(PbrBundle {
                    transform: Transform::from_xyz(x, y, 1.01),
                    mesh: sa.mesh("cell"),
                    material: sa.material("black"),
                    ..Default::default()
                });
            }
            if walkable {
                commands.spawn(PbrBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    mesh: sa.mesh("cell"),
                    material: sa.material("cell"),
                    ..Default::default()
                });
            }

            grid.get_mut(i).unwrap().blocked = blocked;
            grid.get_mut(i).unwrap().walkable = walkable;
        }
    }

    commands.insert_resource(grid);

    // spawn ambient lighting
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.00,
    });

    // spawn player one
    let p = mapbuffer.starting_point.expect("no starting point found");
    let player = commands
        .spawn(Player {
            name: "Player One".into(),
        })
        .id();

    let _e = commands
        .spawn(Token {
            name: "William".into(),
            color: Color::BLUE,
            grid_pos: IVec2 {
                x: p.x as i32,
                y: p.y as i32,
            },
            image: "token_william".into(),
            statblock: "william".into(),
            player: Some(player),
            ..Default::default()
        })
        .id();

    let _e = commands
        .spawn(Token {
            name: "Viktor".into(),
            color: Color::BLUE,
            grid_pos: IVec2 {
                x: p.x as i32 + 1,
                y: p.y as i32,
            },
            image: "token_viktor".into(),
            player: Some(player),
            ..Default::default()
        })
        .id();

    // spawn a goblins
    commands.spawn(Token {
        name: "Goblin 1".into(),
        color: Color::RED,
        grid_pos: IVec2 {
            x: p.x as i32 + 2,
            y: p.y as i32 + 2,
        },
        image: "token_goblin".into(),
        ..Default::default()
    });

    commands.spawn(Token {
        name: "Goblin 2".into(),
        color: Color::RED,
        grid_pos: IVec2 {
            x: p.x as i32 + 4,
            y: p.y as i32 + 3,
        },
        image: "token_goblin".into(),
        ..Default::default()
    });
}

fn on_spawn_token_system(
    mut commands: Commands,
    q: Query<(Entity, &Token), Added<Token>>,
    sa: Res<CommonAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    ass: Res<AssetServer>,
) {
    for (e, token) in q.iter() {
        let handle: Handle<Statblock> = ass.load(format!("statblocks/{}.toml", token.statblock));
        commands
            .entity(e)
            .insert(PbrBundle {
                transform: Transform::from_translation(Token::pos(token.grid_pos)),
                mesh: sa.mesh("token"),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(
                        asset_server.load(format!("images/{}.png", &token.image)),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(handle)
            .with_children(|child_builder| {
                if token.player.is_some() {
                    child_builder.spawn(PointLightBundle {
                        point_light:PointLight {
                            intensity:30.0,
                            range:6.0,
                            shadows_enabled:true,
                            ..Default::default()
                        },
                        transform:Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                        ..Default::default()
                    });
                }
            });
    }
}

fn update_round_command(
    command: &mut RoundCommand,
    _round: &mut ResMut<Round>,
    _time: &Res<Time>,
    tokens: &mut Query<&mut Token>,
    transforms: &mut Query<&mut Transform>,
) {
    match command.variant {
        common::Variant::Nop => {}
        common::Variant::MoveTo { who, to } => {
            let a = command.alpha();
            if let Ok(token) = tokens.get(who) {
                if let Ok(mut transform) = transforms.get_mut(who) {
                    let s = Token::pos(token.grid_pos);
                    let e = Token::pos(to);
                    let v = e - s;
                    let v = v * common::math::smootherstep(0.0, 1.0, a);
                    let mut z = 0.0;
                    if a <= 0.5 {
                        z = a * 2.0;
                    } else {
                        z = 1.0 - (a - 0.5) * 2.0;
                    }
                    z *= 0.2;
                    transform.translation = s + v + Vec3::new(0.0, 0.0, z);
                }
            }
        }
        common::Variant::MoveFar { who: _, to: _ } => {}
        common::Variant::EndTurn { who: _ } => {}
        common::Variant::EndRound {} => {}
        common::Variant::RecvTurn { who: _ } => {}
    }
}

fn finish_round_command(
    command: RoundCommand,
    round: &mut ResMut<Round>,
    _time: &Res<Time>,
    tokens: &mut Query<&mut Token>,
    transforms: &mut Query<&mut Transform>,
    grid: &mut Grid,
) {
    match command.variant {
        common::Variant::Nop => {}
        common::Variant::MoveTo { who, to } => {
            if let Ok(mut token) = tokens.get_mut(who) {
                token.grid_pos = to;
                if let Ok(mut transform) = transforms.get_mut(who) {
                    transform.translation = Token::pos(token.grid_pos);
                }
            }
        }
        common::Variant::MoveFar { who, to } => {
            if let Ok(token) = tokens.get(who) {
                let path = rules::get_path(token, grid, to);
                if !path.is_empty() {
                    for p in path.iter().rev() {
                        round.push_front(RoundCommand::move_to(who, p.to));
                    }
                }
            }
        }
        common::Variant::EndTurn { who } => {
            if round.active_entity == Some(who) {
                round.active_entity = None;
                round.has_taken_turn.insert(who, ());
            }
        }
        common::Variant::EndRound {} => {
            round.has_taken_turn.clear();
            round.active_entity = None;
            round.round_num += 1;
        }
        common::Variant::RecvTurn { who } => {
            round.active_entity = Some(who);
        }
    }
}

fn update_round_command_system(
    mut round: ResMut<Round>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
    tokens: Query<&mut Token>,
) {
    let Some(command) = round.front_mut() else {
        return;
    };
    command.timer_elapsed_sec += time.delta_seconds();
    command.timer_elapsed_sec = command.timer_elapsed_sec.min(command.timer);
    match command.variant {
        common::Variant::Nop => {}
        common::Variant::MoveTo { who, to } => {
            let a = command.alpha();
            if let Ok(token) = tokens.get(who) {
                if let Ok(mut transform) = transforms.get_mut(who) {
                    let s = Token::pos(token.grid_pos);
                    let e = Token::pos(to);
                    let v = e - s;
                    let v = v * common::math::smootherstep(0.0, 1.0, a);
                    let mut z = 0.0;
                    if a <= 0.5 {
                        z = a * 2.0;
                    } else {
                        z = 1.0 - (a - 0.5) * 2.0;
                    }
                    z *= 0.2;
                    transform.translation = s + v + Vec3::new(0.0, 0.0, z);
                }
            }
        }
        common::Variant::MoveFar { who: _, to: _ } => {}
        common::Variant::EndTurn { who: _ } => {}
        common::Variant::EndRound {} => {}
        common::Variant::RecvTurn { who } => {}
    }
}

fn finish_round_command_system(
    mut round: ResMut<Round>,
    mut tokens: Query<&mut Token>,
    mut statblock_handles: Query<&Handle<Statblock>>,
    grid: Res<Grid>,
    mut statblocks: Res<Assets<Statblock>>,
) {
    let Some(command) = round.front_mut() else {
        return;
    };
    if command.timer_elapsed_sec < command.timer {
        return;
    }
    let Some(command) = round.pop_front() else {
        return;
    };

    match command.variant {
        common::Variant::Nop => {}
        common::Variant::MoveTo { who, to } => {
            let Ok(mut token) = tokens.get_mut(who) else {
                return;
            };
            let path = rules::get_path(&token, &grid, to);
            let Some(rc) = path.first() else { return };
            let m = token.movement_ft - rc.cost_ft;
            if m < 0.0 {
                return;
            };
            token.movement_ft = m;
            token.grid_pos = to;
        }
        common::Variant::MoveFar { who, to } => {
            if let Ok(token) = tokens.get(who) {
                let path = rules::get_path(token, &grid, to);
                if !path.is_empty() {
                    for p in path.iter().rev() {
                        round.push_front(RoundCommand::move_to(who, p.to));
                    }
                }
            }
        }
        common::Variant::EndTurn { who: turn_giver } => {
            if round.active_entity == Some(turn_giver) {
                round.active_entity = None;
                round.has_taken_turn.insert(turn_giver, ());
            }
        }
        common::Variant::EndRound {} => {
            round.has_taken_turn.clear();
            round.active_entity = None;
            round.round_num += 1;
        }
        common::Variant::RecvTurn { who } => {
            let Ok(mut token) = tokens.get_mut(who) else {
                return;
            };
            round.active_entity = Some(who);
            let Ok(statblock_handle) = statblock_handles.get(who) else {
                return;
            };
            let Some(statblock) = statblocks.get(statblock_handle) else {
                return;
            };

            token.movement_ft = statblock.speed as f32;
        }
    }
}

fn assign_initiative_system(mut round: ResMut<Round>, tokens: Query<(Entity, &Token)>) {
    if round.is_executing() {
        return;
    }

    // push missing to order
    for (e, _token) in tokens.iter() {
        if !round.initiative_order.contains(&e) {
            round.initiative_order.push(e);
            round.has_taken_turn.insert(e, ());
        }
    }

    // cleanup deleted from the order
    let mut initiative_order = std::mem::take(&mut round.initiative_order);
    for e in initiative_order.drain(..) {
        if tokens.contains(e) {
            round.initiative_order.push(e);
        }
    }
}

fn assign_active_entity_system(
    mut round: ResMut<Round>,
    _tokens: Query<(Entity, &Token)>,
    mut ge: EventWriter<GameEvent>,
) {
    if round.is_executing() {
        return;
    }
    if round.active_entity.is_some() {
        return;
    };

    // no one has the turn, give it to someone or end the round
    let mut next = None;
    for e in round.initiative_order.iter() {
        if !round.has_taken_turn.contains_key(e) {
            ge.send(GameEvent::NextActiveEntity { entity: *e });
            next = Some(*e);
            break;
        }
    }
    match next {
        Some(next) => round.push_front(RoundCommand::recv_turn(next)),
        None => round.push_back(RoundCommand::end_round()),
    }
}

pub fn add_systems(app: &mut App) {
    app.add_systems(Startup, startup_system);
    app.add_systems(
        Update,
        (
            update_round_command_system,
            finish_round_command_system,
            assign_initiative_system,
            assign_active_entity_system,
        )
            .chain(),
    );
    app.add_systems(PostUpdate, on_spawn_token_system);
}
