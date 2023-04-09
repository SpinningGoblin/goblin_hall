use bevy::prelude::Without;

use super::{Builder, Explorer, Gatherer, Miner};

pub type WithoutJob = (
    Without<Miner>,
    Without<Explorer>,
    Without<Builder>,
    Without<Gatherer>,
);
