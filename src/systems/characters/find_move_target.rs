use bevy::{
    prelude::{info, Commands, Entity, Query, Transform, Without},
    utils::Instant,
};
use tdlg::map::layers::{LayerType, StructureType};

use crate::{
    components::{
        characters::Character,
        movement::MoveTarget,
        structures::{Body, Structure},
        GridBox, Map, World,
    },
    resources::config::grid::{grid_coordinate_from_world, pathfind},
};

pub fn find_move_target(
    mut commands: Commands,
    character_query: Query<(&Character, &Transform, Entity), Without<MoveTarget>>,
    structure_query: Query<(&Structure, &Body)>,
    map_query: Query<&Map>,
    world_query: Query<&World>,
) {
    if character_query.is_empty()
        || structure_query.is_empty()
        || map_query.is_empty()
        || world_query.is_empty()
    {
        return;
    }

    let world = world_query.single();
    if !world.tick_just_finished {
        return;
    }

    let (_, transform, entity) = character_query.single();
    let map = map_query.single();

    let character_coordinate = grid_coordinate_from_world(
        &transform.translation.truncate(),
        map.grid_size,
        map.tile_size,
    );

    let visibility_box = GridBox {
        center: character_coordinate.clone(),
        size: 25,
    };

    let instant = Instant::now();

    // For each character, go through the room walls, find the closest, by manhattan distance,
    // and get a path to them.
    let mut coordinate_distances = structure_query
        .iter()
        .filter(|(structure, body)| {
            is_wall(&structure.layer_type) && visibility_box.contains(&body.center_coordinate)
        })
        .collect::<Vec<(&Structure, &Body)>>();
    coordinate_distances
        .sort_by_key(|(_, body)| body.center_coordinate.distance(&character_coordinate));

    for structure_component in coordinate_distances {
        let (structure, body) = structure_component;

        if body.center_coordinate.distance(&character_coordinate) <= 1 {
            return;
        }

        if let Some(mut path) = pathfind(&map, &character_coordinate, &body.center_coordinate) {
            path.reverse();
            let move_target = MoveTarget {
                point: body.cell_center,
                path,
                layer_type: structure.layer_type,
            };

            commands.entity(entity).insert(move_target);
            break;
        } else {
            info!("not found");
        }
    }

    let elapsed = instant.elapsed();
    info!("{:?}", elapsed);
}

fn is_wall(layer_type: &LayerType) -> bool {
    match *layer_type {
        LayerType::Structure(StructureType::Wall) => true,
        _ => false,
    }
}
