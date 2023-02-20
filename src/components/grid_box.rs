use tdlg::map::cells::Coordinate;

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
