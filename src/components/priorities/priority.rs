use super::PriorityLevel;

pub trait Priority {
    fn priority_level(&self) -> PriorityLevel;
}
