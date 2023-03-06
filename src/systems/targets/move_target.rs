use bevy::{
    prelude::{Camera, GlobalTransform, Query, Transform, Vec2, Visibility, With},
    window::{PrimaryWindow, Window},
};

use crate::{
    components::{cameras::GameCamera, target::MouseTarget, Map},
    resources::config::grid::{grid_coordinate_from_world, world_coordinate_from_grid},
};

pub fn move_target(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut target_query: Query<(&mut Transform, &mut Visibility), With<MouseTarget>>,
    map_query: Query<&Map>,
) {
    if target_query.is_empty() || map_query.is_empty() {
        return;
    }

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_query.single();
    let (mut target_transform, mut target_visibility) = target_query.single_mut();

    let Ok(window) = windows.get_single() else {
        return;
    };

    let map = map_query.single();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width(), window.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
        // +20.0 for... unknown reasons. Maybe a "my screen" thing? Need to figure that out.
        let world_coordinate = Vec2::new(world_pos.x + 20.0, world_pos.y + 20.0);
        let grid_coords =
            grid_coordinate_from_world(&world_coordinate, map.grid_size, map.tile_size);
        let new_position = world_coordinate_from_grid(&grid_coords, map.grid_size, map.tile_size);
        *target_visibility = Visibility::Inherited;
        target_transform.translation.x = new_position.x;
        target_transform.translation.y = new_position.y;
        target_transform.translation.z = 2.0;
    } else {
        *target_visibility = Visibility::Hidden;
    }
}
