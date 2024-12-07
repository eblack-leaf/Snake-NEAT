use crate::runner::{Innovation, NodeId};
use foliage::bevy_ecs;
use foliage::bevy_ecs::prelude::Resource;
use std::collections::HashMap;
#[derive(Resource)]
pub(crate) struct ExistingInnovation {
    pub(crate) existing: HashMap<(NodeId, NodeId), Innovation>,
    pub(crate) generator: Innovation,
}

impl ExistingInnovation {
    pub(crate) fn check(&mut self, from: NodeId, to: NodeId) -> Innovation {
        let pair = (from, to);
        if let Some(k) = self.existing.get(&pair) {
            k.clone()
        } else {
            self.generator += 1;
            let idx = self.generator;
            self.existing.insert(pair, idx);
            idx
        }
    }
    pub(crate) fn new(inputs: usize, outputs: usize) -> Self {
        let mut generator = 0;
        let mut existing = HashMap::new();
        // fully-connected innovations
        for i in 0..inputs {
            for o in inputs..(inputs + outputs) {
                existing.insert((i, o), generator);
                generator += 1;
            }
        }
        for i in (inputs + outputs)..(inputs + outputs * 2) {
            for o in inputs..(inputs + outputs) {
                existing.insert((i, o), generator);
                generator += 1;
            }
        }
        Self {
            existing,
            generator,
        }
    }
}
