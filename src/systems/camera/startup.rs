use bevy::prelude::{Camera2dBundle, Commands, Query, Res};

use crate::{
    components::cameras::{CameraMovement, GameCamera},
    resources::config::GameConfiguration,
};

pub fn spawn_camera(
    mut commands: Commands,
    query: Query<&GameCamera>,
    game_config: Res<GameConfiguration>,
) {
    if !query.is_empty() {
        return;
    }

    let mut camera = Camera2dBundle::default();
    camera.transform.scale = game_config.initial_camera_scale();

    // set up a scene to display our texture atlas
    commands
        .spawn(camera)
        .insert(GameCamera)
        .insert(CameraMovement::default());
}
