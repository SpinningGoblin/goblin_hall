use tdlg::map::cells::Coordinate;

use super::movement::Direction;

/// A square of a specific size, in the grid coordinates
pub struct GridBox {
    pub center: Coordinate,
    pub size: i32,
}

impl GridBox {
    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        coordinate.x() < self.max_x()
            && coordinate.x() > self.min_x()
            && coordinate.y() > self.min_y()
            && coordinate.y() < self.max_y()
    }

    pub fn farthest_coordinate_at_direction(&self, direction: &Direction) -> Coordinate {
        match direction {
            Direction::North => Coordinate::from((self.center.x(), self.max_y())),
            Direction::NorthEast => Coordinate::from((self.max_x(), self.max_y())),
            Direction::East => Coordinate::from((self.max_x(), self.center.y())),
            Direction::SouthEast => Coordinate::from((self.max_x(), self.min_y())),
            Direction::South => Coordinate::from((self.center.x(), self.min_y())),
            Direction::SouthWest => Coordinate::from((self.min_x(), self.min_y())),
            Direction::West => Coordinate::from((self.min_x(), self.center.y())),
            Direction::NorthWest => Coordinate::from((self.min_x(), self.max_y())),
        }
    }

    pub fn min_x(&self) -> i32 {
        self.center.x() - self.size
    }

    pub fn max_x(&self) -> i32 {
        self.center.x() + self.size
    }

    pub fn min_y(&self) -> i32 {
        self.center.y() - self.size
    }

    pub fn max_y(&self) -> i32 {
        self.center.y() + self.size
    }
}
