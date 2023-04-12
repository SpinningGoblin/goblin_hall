use bevy::prelude::{Input, KeyCode, Query, Res};

use crate::components::WorldTickCalculation;

pub fn pause_world_tick(
    mut query: Query<&mut WorldTickCalculation>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok(mut world_tick) = query.get_single_mut() else {
        return;
    };

    *world_tick = world_tick.flip();
}
