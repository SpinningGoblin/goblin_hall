#[derive(Clone, Debug, Default)]
pub struct BuilderPriority {
    pub untargeted_storage_setup: bool,
}

impl BuilderPriority {
    pub fn reset(&mut self) {
        self.untargeted_storage_setup = false;
    }
}
