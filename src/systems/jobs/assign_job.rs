use bevy::prelude::{Commands, Entity, Query, Without};

use crate::components::{
    characters::Character,
    jobs::{Builder, Explorer, JobPriority, JobType, Miner},
};

type WithoutJob = (Without<Miner>, Without<Explorer>, Without<Builder>);

pub fn assign_job(
    mut commands: Commands,
    mut query: Query<(&Character, &mut JobPriority, Entity), WithoutJob>,
) {
    // I need to identify what needs to be done for each of the characters.
    // This will be based on the current state of the world and what they can see.
    // For now I'm putting all of this into a single system, and will eventually
    // split this out later if I can find a way to do it.
    for character_bundle in query.iter_mut() {
        let (_, mut job_priority, entity) = character_bundle;

        let mut entity_commands = commands.entity(entity);
        match job_priority.top_priority() {
            JobType::Miner => entity_commands.insert(Miner),
            JobType::Explorer => entity_commands.insert(Explorer),
            JobType::Builder => entity_commands.insert(Builder),
        };

        job_priority.reset();
    }
}
