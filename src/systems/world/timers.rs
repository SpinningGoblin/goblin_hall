use bevy::{
    prelude::{Query, Res},
    time::Time,
};

use crate::components::{World, WorldTimer};

pub fn tick_game_world(time: Res<Time>, mut query: Query<&mut WorldTimer>) {
    if query.is_empty() {
        return;
    }

    let mut timer = query.single_mut();
    timer.tick(time.delta());
}

pub fn check_world_actions(timer_query: Query<&WorldTimer>, mut world_query: Query<&mut World>) {
    if timer_query.is_empty() || world_query.is_empty() {
        return;
    }

    let timer = timer_query.single();
    let mut world = world_query.single_mut();
    world.tick_just_finished = timer.just_finished();
}
