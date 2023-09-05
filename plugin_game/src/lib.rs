use std::f32::consts::PI;

use bevy::prelude::*;
use common::Grid;
use mapgen::{AreaStartingPosition, BspRooms, MapBuilder, SimpleRooms, XStart, YStart};
use rand::{rngs::StdRng, SeedableRng};

fn system_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("textures/cell.png");
    let material_cell = materials.add(StandardMaterial {
        base_color_texture: Some(texture.clone()),
        ..default()
    }); 

    let texture = asset_server.load("textures/brick.png");
    let material_brick = materials.add(StandardMaterial {
        base_color_texture: Some(texture.clone()),
        ..default()
    }); 

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
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: material_brick.clone(),
                    transform: Transform::from_xyz(x, y, 0.5),
                    ..default()
                });
            } 
            if walkable {
                commands.spawn(PbrBundle {
                    transform: Transform::from_xyz(x, y, 0.0).with_rotation(Quat::from_rotation_x(PI  / 2.0)),
                    mesh: meshes.add(shape::Plane::from_size(1.0).into()),
                    
                    material: material_cell.clone(),
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
}


pub struct PluginGame;
impl Plugin for PluginGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system_startup);
    }
}