use bevy::prelude::{Camera2dBundle, Commands, Query, Vec3};

use crate::components::{cameras::GameCamera, movement::CameraMovement};

pub fn spawn_camera(mut commands: Commands, query: Query<&GameCamera>) {
    if !query.is_empty() {
        return;
    }

    let mut camera = Camera2dBundle::default();
    camera.transform.scale = Vec3::splat(2.0);

    // set up a scene to display our texture atlas
    commands
        .spawn(camera)
        .insert(GameCamera)
        .insert(CameraMovement::default());
}
