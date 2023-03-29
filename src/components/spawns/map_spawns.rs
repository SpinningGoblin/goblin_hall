use bevy::prelude::Component;

use super::{TdlgSpawnable, ZoneSpawnable};

#[derive(Component)]
pub struct MapSpawns {
    pub tdlg_spawnables: Vec<TdlgSpawnable>,
    pub zone_spawnables: Vec<ZoneSpawnable>,
}

impl MapSpawns {
    pub fn clear(&mut self) {
        self.tdlg_spawnables.clear();
        self.zone_spawnables.clear();
    }
}
