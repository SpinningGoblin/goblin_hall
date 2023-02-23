use bevy::{
    prelude::{Query, Res},
    time::Time,
};

use crate::components::{World, WorldTimer};

pub fn tick_game_world(
    time: Res<Time>,
    mut query: Query<&mut WorldTimer>,
    mut world_query: Query<&mut World>,
) {
    if query.is_empty() {
        return;
    }

    let mut timer = query.single_mut();
    timer.tick(time.delta());

    if world_query.is_empty() {
        return;
    }

    let mut world = world_query.single_mut();
    world.tick_just_finished = timer.just_finished();
}
