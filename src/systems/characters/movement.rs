use bevy::prelude::{Commands, Entity, Query, Transform};

use crate::components::{characters::Character, movement::MoveTarget, World};

pub fn movement(
    mut commands: Commands,
    world_query: Query<&World>,
    mut character_query: Query<(&Character, &mut Transform, &mut MoveTarget, Entity)>,
) {
    if world_query.is_empty() || character_query.is_empty() {
        return;
    }

    let world = world_query.single();
    if !world.tick_just_finished {
        return;
    }

    let (_, mut transform, mut move_target, entity) = character_query.single_mut();
    if let Some(next_position) = move_target.path.pop() {
        transform.translation.x = next_position.x;
        transform.translation.y = next_position.y;
    } else {
        commands.entity(entity).remove::<MoveTarget>();
    }
}
