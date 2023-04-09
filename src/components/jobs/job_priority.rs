use bevy::prelude::Component;

use super::JobType;

#[derive(Component, Clone, Debug, Default)]
pub struct JobPriority {
    pub explorer: bool,
    pub builder: bool,
    pub miner: bool,
    pub gatherer: bool,
}

impl JobPriority {
    pub fn top_priority(&self) -> JobType {
        if self.builder {
            Some(JobType::Builder)
        } else if self.miner {
            Some(JobType::Miner)
        } else if self.gatherer {
            Some(JobType::Gatherer)
        } else if self.explorer {
            Some(JobType::Explorer)
        } else {
            None
        }
        .unwrap_or_default()
    }

    pub fn reset(&mut self) {
        self.builder = false;
        self.explorer = false;
        self.miner = false;
        self.gatherer = false;
    }
}

#[cfg(test)]
mod tests {
    use crate::components::jobs::JobType;

    use super::JobPriority;

    #[test]
    fn reset() {
        let mut priority = JobPriority {
            builder: true,
            explorer: true,
            miner: true,
            gatherer: true,
        };

        priority.reset();

        assert!(!priority.builder);
        assert!(!priority.explorer);
        assert!(!priority.miner);
        assert!(!priority.gatherer);
    }

    #[test]
    fn default_job_type() {
        let priority = JobPriority::default();

        assert!(matches!(priority.top_priority(), JobType::Explorer));
    }

    #[test]
    fn builder_over_all() {
        let priority = JobPriority {
            miner: true,
            builder: true,
            gatherer: true,
            explorer: true,
        };

        assert!(matches!(priority.top_priority(), JobType::Builder));
    }

    #[test]
    fn miner_over_gatherer() {
        let priority = JobPriority {
            miner: true,
            builder: false,
            gatherer: true,
            explorer: true,
        };

        assert!(matches!(priority.top_priority(), JobType::Miner));
    }

    #[test]
    fn gatherer_over_explorer() {
        let priority = JobPriority {
            miner: false,
            builder: false,
            gatherer: true,
            explorer: true,
        };

        assert!(matches!(priority.top_priority(), JobType::Gatherer));
    }
}
