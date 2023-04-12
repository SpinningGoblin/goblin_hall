use bevy::prelude::{Input, KeyCode, Query, Res};

use crate::components::jobs::{AssignmentMode, GlobalAssignmentMode};

pub fn swap_job_assignment_mode(
    mut query: Query<&mut GlobalAssignmentMode>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(mut global_assignment) = query.get_single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::A) {
        global_assignment.mode = AssignmentMode::Automatic;
    } else if keyboard_input.just_pressed(KeyCode::M) {
        global_assignment.mode = AssignmentMode::Manual;
    }
}
