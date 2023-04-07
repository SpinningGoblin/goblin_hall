use bevy::prelude::{Commands, Entity, Query, Transform, With, Without};

use crate::{
    components::{
        characters::Character,
        jobs::Builder,
        movement::{Path, VisitedPoint},
        structures::GridBody,
        tasks::{SetupStorageArea, Task},
        zones::SetupStorageAreaZone,
        GridBox, Map,
    },
    resources::config::grid::grid_coordinate_from_world,
    utils::movement::path_to_point,
};

type CharacterWithTransform = (&'static Character, Entity, &'static Transform);

type BuilderWithoutTask = (With<Builder>, Without<Task>);

pub fn assign_builder_task(
    mut commands: Commands,
    query: Query<CharacterWithTransform, BuilderWithoutTask>,
    setup_storage_zone_query: Query<(&SetupStorageAreaZone, &GridBody, Entity)>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    let setup_zones = setup_storage_zone_query
        .iter()
        .map(|(_, body, entity)| (body, entity))
        .collect::<Vec<(&GridBody, Entity)>>();
    let mut used_zones: Vec<Entity> = Vec::new();

    for character_bundle in query.iter() {
        let (character, entity, transform) = character_bundle;
        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = character.visibility_box(character_coordinate);

        let possible_setup_zone = setup_zones
            .iter()
            .min_by_key(|(body, _)| body.center_coordinate.distance(&visibility_box.center));

        if let Some((body, setup_entity)) = possible_setup_zone {
            let possible_task = build_task(map, &visibility_box, body, setup_entity);
            if let Some(task) = possible_task {
                used_zones.push(*setup_entity);
                commands.entity(entity).insert(task);
            }
        } else {
            commands.entity(entity).remove::<Builder>();
        }
    }
}

fn build_task(
    map: &Map,
    visibility_box: &GridBox,
    body: &GridBody,
    entity: &Entity,
) -> Option<Task> {
    path_to_point(map, &visibility_box.center, &body.center_coordinate).map(|path| {
        Task::SetupStorageArea(SetupStorageArea {
            done: false,
            entity: Some(*entity),
            coordinate: body.center_coordinate,
            path: Path {
                direction: None,
                points: path
                    .iter()
                    .map(|point| VisitedPoint::from(*point))
                    .collect(),
            },
        })
    })
}
