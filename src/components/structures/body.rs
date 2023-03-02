use bevy::prelude::Component;
use tdlg::map::cells::Coordinate;

#[derive(Component)]
pub struct GridBody {
    pub center_coordinate: Coordinate,
}
