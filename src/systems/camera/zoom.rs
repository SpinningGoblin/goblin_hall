use bevy::prelude::{Input, KeyCode, Mut, Query, Res, Transform, Vec3};

use crate::components::cameras::GameCamera;

pub fn zoom_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&GameCamera, &mut Transform)>,
) {
    if query.is_empty() {
        return;
    }

    let (_, mut transform): (&GameCamera, Mut<Transform>) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Equals) {
        transform.scale = next_zoom_out_scale(transform.scale);
    } else if keyboard_input.just_pressed(KeyCode::Minus) {
        transform.scale = next_zoom_in_scale(transform.scale);
    }
}

fn next_zoom_out_scale(scale: Vec3) -> Vec3 {
    if scale == Vec3::splat(1.0) {
        Vec3::new(2.0, 2.0, 1.0)
    } else if scale == Vec3::new(2.0, 2.0, 1.0) {
        return Vec3::new(4.0, 4.0, 1.0);
    } else if scale == Vec3::new(4.0, 4.0, 1.0) {
        Vec3::new(6.0, 6.0, 1.0)
    } else if scale == Vec3::new(0.25, 0.25, 1.0) {
        Vec3::splat(1.0)
    } else {
        scale
    }
}

fn next_zoom_in_scale(scale: Vec3) -> Vec3 {
    if scale == Vec3::splat(1.0) {
        Vec3::new(0.25, 0.25, 1.0)
    } else if scale == Vec3::new(2.0, 2.0, 1.0) {
        return Vec3::splat(1.0);
    } else if scale == Vec3::new(4.0, 4.0, 1.0) {
        Vec3::new(2.0, 2.0, 1.0)
    } else if scale == Vec3::new(6.0, 6.0, 1.0) {
        Vec3::new(4.0, 4.0, 1.0)
    } else {
        scale
    }
}
