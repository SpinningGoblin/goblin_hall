use strum_macros::EnumIter;

#[derive(PartialEq, Copy, Clone, Debug, EnumIter, Eq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn combine(&self, other: Direction) -> Option<Direction> {
        if self == &other {
            return Some(*self);
        }

        match (self, other) {
            (Direction::North, Direction::East) => Some(Direction::NorthEast),
            (Direction::East, Direction::North) => Some(Direction::NorthEast),
            (Direction::North, Direction::West) => Some(Direction::NorthWest),
            (Direction::West, Direction::North) => Some(Direction::NorthWest),
            (Direction::South, Direction::East) => Some(Direction::SouthEast),
            (Direction::East, Direction::South) => Some(Direction::SouthEast),
            (Direction::South, Direction::West) => Some(Direction::SouthWest),
            (Direction::West, Direction::South) => Some(Direction::SouthWest),
            (Direction::NorthWest, Direction::North) => Some(Direction::NorthWest),
            (Direction::NorthEast, Direction::North) => Some(Direction::NorthEast),
            (Direction::NorthWest, Direction::West) => Some(Direction::NorthWest),
            (Direction::NorthEast, Direction::East) => Some(Direction::NorthEast),
            (Direction::SouthWest, Direction::South) => Some(Direction::SouthWest),
            (Direction::SouthEast, Direction::South) => Some(Direction::SouthEast),
            (Direction::SouthWest, Direction::West) => Some(Direction::SouthWest),
            (Direction::SouthEast, Direction::East) => Some(Direction::SouthEast),
            _ => None,
        }
    }
}
