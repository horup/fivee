use std::f32::consts::PI;

use bevy::prelude::*;
use common::{Grid, CommonAssets, Token, Round, RoundCommand};
use mapgen::{AreaStartingPosition, BspRooms, MapBuilder, SimpleRooms, XStart, YStart};
use rand::{rngs::StdRng, SeedableRng};

fn system_startup(mut commands: Commands, sa:Res<CommonAssets>, mut round:ResMut<Round>) {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let map_size = 64;
    let mapbuffer = MapBuilder::new(map_size, map_size)
        .with(BspRooms::new())
        .with(SimpleRooms::new())
        .with(mapgen::filter::rooms_corridors_nearest::NearestCorridors::new())
        .with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP))
        .build_with_rng(&mut rng);

    let _grid = Grid::new(map_size);
    for y in 0..map_size {
        for x in 0..map_size {
            let blocked = mapbuffer.is_blocked(x, y);
            let walkable = mapbuffer.is_walkable(x, y);
            let x = x as f32 + 0.5;
            let y = y as f32 + 0.5;
            if blocked == true {
                commands.spawn(PbrBundle {
                    mesh: sa.mesh("cube"),
                    material: sa.material("brick"),
                    transform: Transform::from_xyz(x, y, 0.5),
                    ..default()
                });
                commands.spawn(PbrBundle {
                    transform: Transform::from_xyz(x, y, 1.001)
                        .with_rotation(Quat::from_rotation_x(PI / 2.0)),
                    mesh: sa.mesh("tile"),
                    material: sa.material("black"),
                    ..Default::default()
                });
            }
            if walkable {
                commands.spawn(PbrBundle {
                    transform: Transform::from_xyz(x, y, 0.0)
                        .with_rotation(Quat::from_rotation_x(PI / 2.0)),
                        mesh: sa.mesh("tile"),
                        material: sa.material("cell"),
                    ..Default::default()
                });
            }
        }
    }

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

fn spawned_token(mut commands:Commands, q:Query<(Entity, &Token), Added<Token>>, sa:Res<CommonAssets>, mut materials:ResMut<Assets<StandardMaterial>>) {
    for (e, token) in q.iter() {
        commands.entity(e).insert(PbrBundle {
            transform:Transform::from_translation(Token::pos(token.grid_pos) + Vec3::new(0.0, 0.0, 0.5)).with_rotation(Quat::from_rotation_x(PI / 2.0)),
            mesh:sa.mesh("token"),
            material:materials.add(StandardMaterial {
                base_color:token.color.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });
    }
}

fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32
{
    // Scale and clamp x to 0..1 range
    let x = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    // Evaluate polynomial
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

fn update_round_command(command:&mut RoundCommand, round:&mut ResMut<Round>, time:&Res<Time>, tokens:&mut Query<&mut Token>, transforms:&mut Query<&mut Transform>) {
    match command.variant {
        common::Variant::Nop => {},
        common::Variant::MoveTo { who, to } => {
            let a = command.alpha();
            if let Ok(token) = tokens.get(who) {
                if let Ok(mut transform) = transforms.get_mut(who) {
                    let s = Token::pos(token.grid_pos);
                    let e = Token::pos(to);
                    let v = e - s;
                    let v = v * smootherstep(0.0, 1.0, a); 
                    transform.translation = s + v;
                }
            }
        },
    }
}

fn finish_round_command(command:RoundCommand, round:&mut ResMut<Round>, time:&Res<Time>, tokens:&mut Query<&mut Token>, transforms:&mut Query<&mut Transform>) {
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
    }
}


fn update_round(mut round:ResMut<Round>, time:Res<Time>, mut tokens:Query<&mut Token>, mut transforms:Query<&mut Transform>) {
    if let Some(mut command) = round.pop_front() {
        command.timer_elapsed_sec += time.delta_seconds();
        command.timer_elapsed_sec = command.timer_elapsed_sec.min(command.timer);
        update_round_command(&mut command, &mut round, &time, &mut tokens, &mut transforms);
        if command.timer_elapsed_sec >= command.timer {
            finish_round_command(command, &mut round, &time, &mut tokens, &mut transforms);
        } else {
            round.push_front_command(command);
        }
    }
}

pub struct PluginGame;
impl Plugin for PluginGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system_startup);
        app.add_systems(Update, update_round);
        app.add_systems(PostUpdate, (spawned_token, update_round));
    }
}
