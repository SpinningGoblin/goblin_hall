use bevy::{
    prelude::{info, Query, Res, ResMut},
    time::Time,
};

use crate::{components::World, resources::config::WorldTickTimer};

pub fn tick_game_world(
    time: Res<Time>,
    mut timer: ResMut<WorldTickTimer>,
    mut world_query: Query<&mut World>,
) {
    if world_query.is_empty() {
        return;
    }

    let mut world = world_query.single_mut();
    world.tick_just_finished = timer.0.tick(time.delta()).just_finished();

    if world.tick_just_finished {
        info!("world tick finished")
    }
}
