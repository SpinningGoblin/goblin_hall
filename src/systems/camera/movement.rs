use bevy::prelude::{Mut, Query, Transform};

use crate::components::{cameras::GameCamera, movement::CameraMovement};

pub fn move_camera(mut query: Query<(&GameCamera, &CameraMovement, &mut Transform)>) {
    if query.is_empty() {
        return;
    }

    let (_, movement, mut transform): (&GameCamera, &CameraMovement, Mut<Transform>) =
        query.single_mut();

    let x = movement.speed.current.x + transform.translation.x;
    let y = movement.speed.current.y + transform.translation.y;

    transform.translation.x = x;
    transform.translation.y = y;
}
