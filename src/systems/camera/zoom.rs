use bevy::prelude::{Input, KeyCode, Query, Res, Transform};

use crate::{components::cameras::GameCamera, resources::config::GameConfiguration};

pub fn zoom_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&GameCamera, &mut Transform)>,
    game_config: Res<GameConfiguration>,
) {
    let Ok((_, mut transform)) = query.get_single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Equals) {
        if let Some(scale) = game_config.zoom_in_level(&transform.scale) {
            transform.scale = scale;
        }
    } else if keyboard_input.just_pressed(KeyCode::Minus) {
        if let Some(scale) = game_config.zoom_out_level(&transform.scale) {
            transform.scale = scale;
        }
    }
}
