pub struct BoundingBox {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl BoundingBox {
    pub fn intersects(&self, bounding_box: &BoundingBox) -> bool {
        self.min_x < bounding_box.max_x
            && self.max_x > bounding_box.min_x
            && self.max_y > bounding_box.min_y
            && self.min_y < bounding_box.max_y
    }

    pub fn square(x: f32, y: f32, width: f32) -> Self {
        let half_width = (width / 2.0).floor();
        Self {
            min_x: x - half_width,
            max_x: x + half_width,
            min_y: y - half_width,
            max_y: y + half_width,
        }
    }
}
