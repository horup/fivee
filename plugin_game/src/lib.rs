use std::f32::consts::PI;

use bevy::prelude::*;
use common::{Grid, CommonAssets, Token};
use mapgen::{AreaStartingPosition, BspRooms, MapBuilder, SimpleRooms, XStart, YStart};
use rand::{rngs::StdRng, SeedableRng};

fn system_startup(mut commands: Commands, sa:Res<CommonAssets>) {
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
    commands.spawn(Token {
        color: Color::BLUE,
        grid_pos: IVec2 { x: p.x as i32, y: p.y as i32 },
    });

    // spawn a goblin
    commands.spawn(Token {
        color: Color::RED,
        grid_pos: IVec2 { x: p.x as i32 + 2, y: p.y as i32 + 2 },
    });
}

fn token_spawned(mut commands:Commands, mut q:Query<(Entity, &Token), Added<Token>>, sa:Res<CommonAssets>, mut materials:ResMut<Assets<StandardMaterial>>) {
    for (e, token) in q.iter() {
        commands.entity(e).insert(PbrBundle {
            transform:Transform::from_translation(token.pos() + Vec3::new(0.0, 0.0, 0.5)).with_rotation(Quat::from_rotation_x(PI / 2.0)),
            mesh:sa.mesh("token"),
            material:materials.add(StandardMaterial {
                base_color:token.color.clone(),
                ..Default::default()
            }),
            ..Default::default()
        });
    }
}

pub struct PluginGame;
impl Plugin for PluginGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system_startup);
        app.add_systems(PostUpdate, token_spawned);
    }
}
