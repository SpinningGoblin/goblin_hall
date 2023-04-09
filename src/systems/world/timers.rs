use bevy::prelude::{Query, Res, ResMut, Time};

use crate::{components::World, resources::config::WorldTickTimer};

pub fn tick_game_world(
    time: Res<Time>,
    mut timer: ResMut<WorldTickTimer>,
    mut world_query: Query<&mut World>,
) {
    let Ok(mut world) = world_query.get_single_mut() else {
        return;
    };
    world.tick_just_finished = timer.0.tick(time.delta()).just_finished();
}

pub fn tick_just_finished(world_query: Query<&World>) -> bool {
    match world_query.get_single() {
        Ok(world) => world.tick_just_finished,
        _ => false,
    }
}
