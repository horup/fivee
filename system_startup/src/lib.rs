use bevy::prelude::*;
use mapgen::{AreaStartingPosition, BspRooms, MapBuilder, SimpleRooms, XStart, YStart};
use rand::{rngs::StdRng, SeedableRng};

pub fn system_startup(mut commands:Commands,  mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let map_size = 64;
    let mapbuffer = MapBuilder::new(map_size, map_size)
        .with(BspRooms::new())
        .with(SimpleRooms::new())
        .with(mapgen::filter::rooms_corridors_nearest::NearestCorridors::new())
        .with(AreaStartingPosition::new(XStart::LEFT, YStart::TOP))
        .build_with_rng(&mut rng);


     // spawn camera
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });


}
