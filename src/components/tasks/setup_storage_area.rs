use tdlg::map::cells::Coordinate;

#[derive(Clone)]
pub struct SetupStorageArea {
    pub done: bool,
    pub coordinate: Coordinate,
}
