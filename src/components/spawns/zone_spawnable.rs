use crate::components::zones::ZoneType;

use super::SpawnCoordinate;

pub struct ZoneSpawnable {
    pub spawn_coordinate: SpawnCoordinate,
    pub zone_type: ZoneType,
}
