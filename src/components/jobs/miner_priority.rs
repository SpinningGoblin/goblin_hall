#[derive(Clone, Debug, Default)]
pub struct MinerPriority {
    pub closest_mineable_distance: Option<u128>,
}

impl MinerPriority {
    pub fn reset(&mut self) {
        self.closest_mineable_distance = None;
    }
}
