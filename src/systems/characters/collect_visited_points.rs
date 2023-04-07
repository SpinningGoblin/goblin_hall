use bevy::prelude::{EventReader, Query};

use crate::{components::jobs::ExplorationHistory, events::PointVisited};

pub fn collect_visited_points(
    mut events: EventReader<PointVisited>,
    mut query: Query<&mut ExplorationHistory>,
) {
    let Ok(mut exploration_history) = query.get_single_mut() else {
        return;
    };

    for event in events.iter() {
        exploration_history.push(event.point);
    }
}
