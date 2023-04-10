use bevy::prelude::Component;

use super::{BuilderPriority, ExplorerPriority, GathererPriority, JobType, MinerPriority};

#[derive(Component, Clone, Debug, Default)]
pub struct JobPriority {
    pub explorer: ExplorerPriority,
    pub builder: BuilderPriority,
    pub miner: MinerPriority,
    pub gatherer: GathererPriority,
}

impl JobPriority {
    pub fn top_priority(&self) -> JobType {
        if self.builder.untargeted_storage_setup {
            JobType::Builder
        } else if self.gatherer.has_full_resource_inventory {
            JobType::Gatherer
        } else {
            match (
                self.miner.closest_mineable_distance,
                self.gatherer.closest_gatherable_distance,
            ) {
                (None, None) => {
                    if self.explorer.untargeted_zone {
                        JobType::Explorer
                    } else {
                        JobType::default()
                    }
                }
                (None, Some(_)) => JobType::Gatherer,
                (Some(_), None) => JobType::Miner,
                (Some(mineable_distance), Some(gatherable_distance)) => {
                    if mineable_distance >= gatherable_distance {
                        JobType::Miner
                    } else {
                        JobType::Gatherer
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.builder.reset();
        self.explorer.reset();
        self.gatherer.reset();
        self.miner.reset();
    }
}

#[cfg(test)]
mod tests {
    use crate::components::jobs::{
        BuilderPriority, ExplorerPriority, GathererPriority, JobType, MinerPriority,
    };

    use super::JobPriority;

    #[test]
    fn reset() {
        let mut priority = JobPriority {
            builder: BuilderPriority {
                untargeted_storage_setup: true,
            },
            explorer: ExplorerPriority {
                untargeted_zone: true,
            },
            miner: MinerPriority {
                closest_mineable_distance: Some(1),
            },
            gatherer: GathererPriority {
                closest_gatherable_distance: Some(1),
                has_full_resource_inventory: true,
            },
        };

        priority.reset();

        assert!(!priority.builder.untargeted_storage_setup);
        assert!(!priority.explorer.untargeted_zone);
        assert!(priority.miner.closest_mineable_distance.is_none());
        assert!(priority.gatherer.closest_gatherable_distance.is_none());
        assert!(!priority.gatherer.has_full_resource_inventory);
    }

    #[test]
    fn default_job_type() {
        let priority = JobPriority::default();

        assert!(matches!(priority.top_priority(), JobType::Explorer));
    }

    #[test]
    fn builder_over_all() {
        let priority = JobPriority {
            builder: BuilderPriority {
                untargeted_storage_setup: true,
            },
            explorer: ExplorerPriority {
                untargeted_zone: true,
            },
            miner: MinerPriority {
                closest_mineable_distance: Some(1),
            },
            gatherer: GathererPriority {
                closest_gatherable_distance: Some(1),
                has_full_resource_inventory: true,
            },
        };

        assert!(matches!(priority.top_priority(), JobType::Builder));
    }

    #[test]
    fn miner_over_gatherer() {
        let priority = JobPriority {
            builder: BuilderPriority {
                untargeted_storage_setup: false,
            },
            explorer: ExplorerPriority {
                untargeted_zone: true,
            },
            miner: MinerPriority {
                closest_mineable_distance: Some(1),
            },
            gatherer: GathererPriority {
                closest_gatherable_distance: Some(1),
                has_full_resource_inventory: false,
            },
        };

        assert!(matches!(priority.top_priority(), JobType::Miner));
    }

    #[test]
    fn gatherer_over_explorer() {
        let priority = JobPriority {
            builder: BuilderPriority {
                untargeted_storage_setup: false,
            },
            explorer: ExplorerPriority {
                untargeted_zone: true,
            },
            miner: MinerPriority {
                closest_mineable_distance: Some(1),
            },
            gatherer: GathererPriority {
                closest_gatherable_distance: Some(2),
                has_full_resource_inventory: false,
            },
        };

        assert!(matches!(priority.top_priority(), JobType::Gatherer));
    }

    #[test]
    fn gatherer_if_full_inventory_over_miner() {
        let priority = JobPriority {
            builder: BuilderPriority {
                untargeted_storage_setup: false,
            },
            explorer: ExplorerPriority {
                untargeted_zone: true,
            },
            miner: MinerPriority {
                closest_mineable_distance: Some(10),
            },
            gatherer: GathererPriority {
                closest_gatherable_distance: None,
                has_full_resource_inventory: true,
            },
        };

        assert!(matches!(priority.top_priority(), JobType::Gatherer));
    }
}
