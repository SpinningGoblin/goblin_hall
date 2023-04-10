use bevy::prelude::{Local, Query, Transform, Without};

use crate::components::{
    cameras::{CameraMovement, GameCamera},
    characters::Character,
};

#[derive(Default)]
pub struct MovedToCharacterOnce {
    moved: bool,
}

pub fn move_camera(
    mut query: Query<(&GameCamera, &CameraMovement, &mut Transform)>,
    character_query: Query<(&Character, &Transform), Without<GameCamera>>,
    mut moved_to_character_once: Local<MovedToCharacterOnce>,
) {
    let Ok((_, movement, mut transform)) = query.get_single_mut() else {
        return;
    };

    let character_bundle = character_query.iter().next();
    let Some((_, character_transform)) = character_bundle else {
        return;
    };

    let x = movement.speed.current.x + transform.translation.x;
    let y = movement.speed.current.y + transform.translation.y;

    transform.translation.x = x;
    transform.translation.y = y;

    if moved_to_character_once.moved {
        return;
    }

    transform.translation.x = character_transform.translation.x;
    transform.translation.y = character_transform.translation.y;

    moved_to_character_once.moved = true;
}
