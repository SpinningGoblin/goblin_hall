use bevy::prelude::{Query, Transform, Visibility, With};

use crate::{
    components::{characters::Character, structures::GridBody, GridBox, Map},
    resources::config::grid::grid_coordinate_from_world,
};

pub fn show_in_visible_area(
    query: Query<&Transform, With<Character>>,
    mut visible_bodies: Query<(&mut Visibility, &GridBody)>,
    map_query: Query<&Map>,
) {
    if map_query.is_empty() {
        return;
    }

    let map = map_query.single();

    let visibility_boxes: Vec<GridBox> = query
        .iter()
        .map(|transform| {
            let coordinate = grid_coordinate_from_world(
                &transform.translation.truncate(),
                map.grid_size,
                map.tile_size,
            );
            GridBox {
                center: coordinate,
                size: 10,
            }
        })
        .collect();

    for visibility_bundle in visible_bodies.iter_mut().filter(|(_, body)| {
        visibility_boxes
            .iter()
            .any(|visibility_box| visibility_box.contains(&body.center_coordinate))
    }) {
        let (mut visibility, _) = visibility_bundle;
        *visibility = Visibility::Inherited;
    }
}
