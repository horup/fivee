use bevy::{prelude::*, input::mouse::MouseWheel, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};
use common::{UIDebugFPS, WorldCursor, CommonAssets};
fn system_ui_startup(mut commands:Commands, common_assets:ResMut<CommonAssets>) {
    // spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0,0.0,32.0).looking_at(Vec3::new(0.0, 16.0, 0.0), Vec3::Y),
        ..default()
    });

    let font = common_assets.font("default");
    commands.spawn(
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
    ).insert(UIDebugFPS);


    // spawn world cusor 
    commands.spawn(PbrBundle {
        mesh:common_assets.mesh("selector"),
        material:common_assets.material("white"),
        ..Default::default()
    }).insert(WorldCursor::default());
}

fn update_camera(keys: Res<Input<KeyCode>>, mut camera:Query<(&mut Camera3d, &mut Transform, )>, time:Res<Time>, mut mouse_wheel:EventReader<MouseWheel>) {
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

fn update_debug(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<UIDebugFPS>>) {
    for mut text in &mut query {
        if let Some(fps_diagnostics) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            text.sections[0].value = format!("{}", fps_diagnostics.smoothed().unwrap_or_default() as i32);
        }
    }
}

fn update_world_cursor(mut cursor_moved_events: EventReader<CursorMoved>, mut query_camera: Query<(&GlobalTransform, &Camera)>, mut world_cursor:Query<(&mut WorldCursor, &mut Transform)>, mouse:Res<Input<MouseButton>>) {
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
                world_cursor_transform.translation = grid_pos.as_vec2().extend(0.0) + Vec3::new(0.5, 0.5, 0.0);
            }
        }
    }

    if mouse.just_pressed(MouseButton::Left) {
        
    }
}



pub struct PluginUI;
impl Plugin for PluginUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system_ui_startup);
        app.add_systems(PreUpdate, (update_camera, update_debug, update_world_cursor));
    }
}