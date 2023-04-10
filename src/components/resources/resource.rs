#[derive(Clone, Debug)]
pub enum Resource {
    Stone(u8),
}

#[derive(Clone)]
pub enum ResourceBundle {
    Stone(u16),
}
