use bevy::prelude::{Input, KeyCode, Mut, Query, Res, Transform};

use crate::{components::cameras::GameCamera, resources::config::GameConfiguration};

pub fn zoom_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&GameCamera, &mut Transform)>,
    game_config: Res<GameConfiguration>,
) {
    if query.is_empty() {
        return;
    }

    let (_, mut transform): (&GameCamera, Mut<Transform>) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Equals) {
        if let Some(scale) = game_config.zoom_out_level(&transform.scale) {
            transform.scale = scale;
        }
    } else if keyboard_input.just_pressed(KeyCode::Minus) {
        if let Some(scale) = game_config.zoom_in_level(&transform.scale) {
            transform.scale = scale;
        }
    }
}
