use bevy::prelude::{Query, Res, ResMut, Time};

use crate::{
    components::{World, WorldTickCalculation},
    resources::config::WorldTickTimer,
};

pub fn tick_game_world(
    time: Res<Time>,
    mut timer: ResMut<WorldTickTimer>,
    mut world_query: Query<&mut World>,
    world_calculation_query: Query<&WorldTickCalculation>,
) {
    let (Ok(mut world), Ok(world_calculation)) = (
        world_query.get_single_mut(),
        world_calculation_query.get_single(),
    ) else {
        return;
    };

    if matches!(world_calculation, WorldTickCalculation::Paused) {
        return;
    }

    world.tick_just_finished = timer.0.tick(time.delta()).just_finished();
}

pub fn tick_just_finished(world_query: Query<&World>) -> bool {
    match world_query.get_single() {
        Ok(world) => world.tick_just_finished,
        _ => false,
    }
}
