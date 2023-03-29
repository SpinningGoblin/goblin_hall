use tdlg::map::cells::Coordinate;

#[derive(Clone, Debug)]
pub struct SpawnCoordinate {
    pub coordinate: Coordinate,
    pub z_level: f32,
}
