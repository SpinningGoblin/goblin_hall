use bevy::prelude::{Commands, Entity, Query, Transform};

use crate::{
    components::{
        characters::Character,
        jobs::{Builder, Job, ManualAssignment},
        movement::{Path, VisitedPoint},
        structures::GridBody,
        tasks::{SetupStorageArea, SetupStorageAreaTask, WithoutTask},
        zones::SetupStorageAreaZone,
        GridBox, Map,
    },
    resources::config::grid::grid_coordinate_from_world,
    utils::movement::path_to_point,
};

type BuilderWithTransform = (
    &'static Character,
    Entity,
    &'static Transform,
    &'static Builder,
    &'static ManualAssignment,
);

pub fn assign_builder_task(
    mut commands: Commands,
    query: Query<BuilderWithTransform, WithoutTask>,
    mut setup_storage_zone_query: Query<(&mut SetupStorageAreaZone, &GridBody, Entity)>,
    map_query: Query<&Map>,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };

    let mut used_zones: Vec<Entity> = Vec::new();

    for character_bundle in query.iter() {
        let (character, entity, transform, builder, manual_assignment) = character_bundle;
        let character_coordinate = grid_coordinate_from_world(
            &transform.translation.truncate(),
            map.grid_size,
            map.tile_size,
        );

        let visibility_box = character.visibility_box(character_coordinate);

        let possible_setup_zone = setup_storage_zone_query
            .iter_mut()
            .filter(|(setup_zone, _, entity)| !setup_zone.targeted && !used_zones.contains(entity))
            .min_by_key(|(_, body, _)| body.center_coordinate.distance(&visibility_box.center));

        let mut entity_commands = commands.entity(entity);
        if let Some((mut zone, body, setup_entity)) = possible_setup_zone {
            let possible_task = build_task(map, &visibility_box, body, setup_entity);
            if let Some(task) = possible_task {
                zone.targeted = true;
                used_zones.push(setup_entity);
                entity_commands.insert(task);
            } else if builder.is_automatically_assigned() {
                entity_commands.remove::<Builder>();
            }
        } else if builder.is_automatically_assigned() || manual_assignment.will_reassign() {
            entity_commands.remove::<Builder>();
        }
    }
}

fn build_task(
    map: &Map,
    visibility_box: &GridBox,
    body: &GridBody,
    entity: Entity,
) -> Option<SetupStorageAreaTask> {
    path_to_point(map, &visibility_box.center, &body.center_coordinate).map(|path| {
        SetupStorageAreaTask {
            setup_area: SetupStorageArea {
                done: false,
                entity: Some(entity),
                coordinate: body.center_coordinate,
                path: Path {
                    direction: None,
                    points: path
                        .iter()
                        .map(|point| VisitedPoint::from(*point))
                        .collect(),
                },
            },
        }
    })
}
