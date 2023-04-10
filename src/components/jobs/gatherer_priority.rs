#[derive(Clone, Debug, Default)]
pub struct GathererPriority {
    pub closest_gatherable_distance: Option<u128>,
    pub has_full_resource_inventory: bool,
}

impl GathererPriority {
    pub fn reset(&mut self) {
        self.closest_gatherable_distance = None;
        self.has_full_resource_inventory = false;
    }
}
