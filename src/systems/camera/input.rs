use bevy::{
    prelude::{Input, KeyCode, Mut, Query, Res, Transform, Vec2},
    time::Time,
};

use crate::{
    components::{
        cameras::{CameraMovement, GameCamera},
        movement::{CameraMoveTimer, Direction},
    },
    resources::config::GameConfiguration,
};

use super::speed::{x_axis_speed, y_axis_speed};

pub fn process_movement_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&GameCamera, &mut CameraMovement, &Transform)>,
    game_config: Res<GameConfiguration>,
    mut timer_query: Query<&mut CameraMoveTimer>,
) {
    if query.is_empty() || timer_query.is_empty() {
        return;
    }

    let (_, mut movement, transform): (&GameCamera, Mut<CameraMovement>, &Transform) =
        query.single_mut();

    let mut camera_move_timer = timer_query.single_mut();

    movement.speed.current = Vec2::ZERO;
    movement.direction = None;

    let speed_modifier = game_config.camera_movement_modifier(&transform.scale);

    if keyboard_input.just_pressed(KeyCode::Left) {
        movement.speed.current -= x_axis_speed(game_config.tile_size(), speed_modifier);
        movement.add_direction(Direction::West);
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        movement.speed.current += x_axis_speed(game_config.tile_size(), speed_modifier);
        movement.add_direction(Direction::East);
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        movement.speed.current += y_axis_speed(game_config.tile_size(), speed_modifier);
        movement.add_direction(Direction::North);
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        movement.speed.current -= y_axis_speed(game_config.tile_size(), speed_modifier);
        movement.add_direction(Direction::South);
    }

    if movement.speed.current.x != 0.0 || movement.speed.current.y != 0.0 {
        camera_move_timer.reset_movement_timer();
        return;
    }

    if camera_move_timer
        .tick_movement_timer(time.delta())
        .just_finished()
    {
        if keyboard_input.pressed(KeyCode::Left) {
            movement.speed.current -= x_axis_speed(game_config.tile_size(), speed_modifier);
            movement.add_direction(Direction::West);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            movement.speed.current += x_axis_speed(game_config.tile_size(), speed_modifier);
            movement.add_direction(Direction::East);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            movement.speed.current += y_axis_speed(game_config.tile_size(), speed_modifier);
            movement.add_direction(Direction::North);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            movement.speed.current -= y_axis_speed(game_config.tile_size(), speed_modifier);
            movement.add_direction(Direction::South);
        }
    }
}
