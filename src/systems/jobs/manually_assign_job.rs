use bevy::prelude::{Input, KeyCode, Query, Res, Transform, Visibility, With};

use crate::{
    components::{
        characters::Character,
        jobs::{JobType, ManualAssignment},
        target::MouseTarget,
        Map,
    },
    resources::config::grid::grid_coordinate_from_world,
};

pub fn manually_assign_job(
    keyboard_input: Res<Input<KeyCode>>,
    target_query: Query<(&Transform, &Visibility), With<MouseTarget>>,
    mut character_query: Query<(&Transform, &mut ManualAssignment), With<Character>>,
    map_query: Query<&Map>,
) {
    let queries = (target_query.get_single(), map_query.get_single());
    let (Ok((target_transform, visibility)), Ok(map)) = queries else {
        return;
    };

    if matches!(visibility, Visibility::Hidden) {
        return;
    }

    let target_keys = vec![KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4];
    if !keyboard_input.any_just_pressed(target_keys) {
        return;
    }

    let target_point = target_transform.translation.truncate();
    let target_coordinate = grid_coordinate_from_world(&target_point, map.grid_size, map.tile_size);

    for (transform, mut manual_assignment) in character_query.iter_mut() {
        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        if character_coordinate.eq(&target_coordinate) {
            let job = if keyboard_input.just_pressed(KeyCode::Key1) {
                Some(JobType::Builder)
            } else if keyboard_input.just_pressed(KeyCode::Key2) {
                Some(JobType::Explorer)
            } else if keyboard_input.just_pressed(KeyCode::Key3) {
                Some(JobType::Gatherer)
            } else if keyboard_input.just_pressed(KeyCode::Key4) {
                Some(JobType::Miner)
            } else {
                None
            };

            manual_assignment.job = job;
        }
    }
}
