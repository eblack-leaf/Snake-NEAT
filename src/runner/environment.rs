use crate::runner::compatibility::CompatibilityFactors;
use foliage::bevy_ecs;
use foliage::bevy_ecs::prelude::Resource;
#[derive(Resource)]
pub(crate) struct Environment {
    pub(crate) population_count: i32,
    pub(crate) input_size: usize,
    pub(crate) output_size: usize,
    pub(crate) compatibility_factors: CompatibilityFactors,
    pub(crate) compatibility_threshold: f32,
    // other configurations
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            population_count: 0,
            input_size: 0,
            output_size: 0,
            compatibility_factors: CompatibilityFactors {
                c1: 0.0,
                c2: 0.0,
                c3: 0.0,
            },
            compatibility_threshold: 0.0,
        }
    }
}
