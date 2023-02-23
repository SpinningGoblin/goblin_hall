use crate::components::movement::Path;

#[allow(dead_code)]
pub enum Task {
    Walk(Path),
}

impl Task {
    pub fn is_complete(&self) -> bool {
        match self {
            Task::Walk(path) => path.points.is_empty(),
        }
    }
}
