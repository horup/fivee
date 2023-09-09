use bevy::{prelude::*, input::mouse::MouseWheel};

pub fn camera_system(
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