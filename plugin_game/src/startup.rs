

use bevy::prelude::*;
use common::{Grid, CommonAssets, Token, Round, RoundCommand};
use mapgen::{AreaStartingPosition, BspRooms, MapBuilder, SimpleRooms, XStart, YStart};
use rand::{rngs::StdRng, SeedableRng};


pub fn startup(mut commands: Commands, sa:Res<CommonAssets>, mut round:ResMut<Round>) {
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