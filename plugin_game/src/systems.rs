use bevy::prelude::*;
use common::{CommonAssets, Token, Round, RoundCommand, Grid};
use mapgen::{BspRooms, SimpleRooms, MapBuilder, XStart, YStart, AreaStartingPosition};
use rand::{rngs::StdRng, SeedableRng};

pub fn startup_system(mut commands: Commands, sa:Res<CommonAssets>, mut round:ResMut<Round>) {
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
            let i = IVec2 { x: x as i32, y: y as i32 };
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

    // spawn lighting
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });

    // spawn player
    let p = mapbuffer.starting_point.expect("no starting point found");
    let e = commands.spawn(Token {
        color: Color::BLUE,
        grid_pos: IVec2 { x: p.x as i32, y: p.y as i32 },
    }).id();

    round.push_front_command(RoundCommand::move_to(e, IVec2 { x: p.x as i32 + 1, y: p.y as i32 }));

    // spawn a goblin
    commands.spawn(Token {
        color: Color::RED,
        grid_pos: IVec2 { x: p.x as i32 + 2, y: p.y as i32 + 2 },
    });
}


pub fn on_spawn_token_system(mut commands:Commands, q:Query<(Entity, &Token), Added<Token>>, sa:Res<CommonAssets>, mut materials:ResMut<Assets<StandardMaterial>>) {
    for (e, token) in q.iter() {
        commands.entity(e).insert(PbrBundle {
            transform:Transform::from_translation(Token::pos(token.grid_pos)),
            mesh:sa.mesh("token"),
            material:materials.add(StandardMaterial {
                base_color:token.color,
                ..Default::default()
            }),
            ..Default::default()
        });
    }
}

pub fn update_round_command(command:&mut RoundCommand, round:&mut ResMut<Round>, _time:&Res<Time>, tokens:&mut Query<&mut Token>, transforms:&mut Query<&mut Transform>) {
    match command.variant {
        common::Variant::Nop => {},
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
                    z*=0.2;
                    transform.translation = s + v + Vec3::new(0.0, 0.0, z);
                }
            }
        },
        common::Variant::MoveFar { who, to } => {
            
        },
    }
}

pub fn finish_round_command(command:RoundCommand, round:&mut ResMut<Round>, _time:&Res<Time>, tokens:&mut Query<&mut Token>, transforms:&mut Query<&mut Transform>, grid:&mut Grid) {
    match command.variant {
        common::Variant::Nop => {},
        common::Variant::MoveTo { who, to } => {
            if let Ok(mut token) = tokens.get_mut(who) {
                token.grid_pos = to;
                if let Ok(mut transform) = transforms.get_mut(who) {
                    transform.translation = Token::pos(token.grid_pos);
                }
            }
        },
        common::Variant::MoveFar { who, to } => {
            if let Ok(token) = tokens.get(who) {
                let path = rules::get_path(token, &grid, to);
                if path.len() > 0 {
                    for p in path {
                        round.push_back_command(RoundCommand::move_to(who, p.to));
                    }
                }
            }
        },
    }
}

pub fn update_round_system(mut round:ResMut<Round>, time:Res<Time>, mut tokens:Query<&mut Token>, mut transforms:Query<&mut Transform>, mut grid:ResMut<Grid>) {
    if let Some(mut command) = round.pop_front() {
        command.timer_elapsed_sec += time.delta_seconds();
        command.timer_elapsed_sec = command.timer_elapsed_sec.min(command.timer);
        update_round_command(&mut command, &mut round, &time, &mut tokens, &mut transforms);
        if command.timer_elapsed_sec >= command.timer {
            finish_round_command(command, &mut round, &time, &mut tokens, &mut transforms, &mut grid);
        } else {
            round.push_front_command(command);
        }
    }
}


pub struct PluginGame;
impl Plugin for PluginGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, update_round_system);
        app.add_systems(PostUpdate, on_spawn_token_system);
    }
}
