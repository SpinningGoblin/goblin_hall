#[derive(Clone, Debug, Default)]
pub struct ExplorerPriority {
    pub untargeted_zone: bool,
}

impl ExplorerPriority {
    pub fn reset(&mut self) {
        self.untargeted_zone = false;
    }
}
