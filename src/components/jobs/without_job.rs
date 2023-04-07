use bevy::prelude::Without;

use super::{Builder, Explorer, Miner};

pub type WithoutJob = (Without<Miner>, Without<Explorer>, Without<Builder>);
