use crate::runner::compatibility::Compatibility;
use crate::runner::connection::Connection;
use crate::runner::environment::Environment;
use crate::runner::genome::Genome;
use crate::runner::node::Node;
use crate::runner::{Fitness, Generation, Runner, SpeciesId};
use foliage::bevy_ecs;
use foliage::bevy_ecs::change_detection::Res;
use foliage::bevy_ecs::component::Component;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::event::Event;
use foliage::bevy_ecs::observer::Trigger;
use foliage::bevy_ecs::prelude::Query;
use foliage::tree::Tree;
#[derive(Component, Clone)]
pub(crate) struct Species {
    pub(crate) id: SpeciesId,
    pub(crate) members: Vec<Entity>,
    pub(crate) last_improved: Generation,
    pub(crate) representative: Entity,
    pub(crate) max_fitness: Fitness,
    pub(crate) shared_fitness: Fitness,
    pub(crate) percent_total: f32,
}

#[derive(Event)]
pub(crate) struct Speciate {}

impl Speciate {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        runner: Res<Runner>,
        mut species: Query<&mut Species>,
        mut population: Query<&mut Genome>,
        mut connections: Query<&mut Connection>,
        mut nodes: Query<&mut Node>,
        environment: Res<Environment>,
    ) {
        for s in runner.species.iter() {
            species.get_mut(*s).unwrap().members.clear()
        }
        for genome in runner
            .population
            .iter()
            .map(|p| population.get(*p).unwrap())
        {
            let mut found = None;
            for s in runner
                .species
                .iter()
                .cloned()
                .map(|s| species.get(s).unwrap())
            {
                let repr = population.get(s.representative).unwrap();
                let mut compatibility = Compatibility::new();
                let repr_innovation_max = repr
                    .connections
                    .iter()
                    .map(|c| connections.get(*c).unwrap().innovation)
                    .max()
                    .unwrap_or_default();
                let mut num_weights = 0.0;
                for conn in genome
                    .connections
                    .iter()
                    .map(|c| connections.get(*c).unwrap())
                {
                    let matching = repr
                        .connections
                        .iter()
                        .map(|c| connections.get(*c).unwrap())
                        .find(|c| c.innovation == conn.innovation);
                    if let Some(matching) = matching {
                        compatibility.weight_difference += conn.weight - matching.weight;
                        num_weights += 1.0;
                    }
                    if conn.innovation > repr_innovation_max {
                        compatibility.excess += 1.0;
                    } else if matching.is_none() {
                        compatibility.disjoint += 1.0;
                    }
                }
                let repr_node_max = repr
                    .nodes
                    .iter()
                    .map(|n| nodes.get(*n).unwrap())
                    .max_by(|a, b| a.id.partial_cmp(&b.id).unwrap())
                    .unwrap_or(&Node::new())
                    .id;
                for node in genome.nodes.iter().map(|n| nodes.get(*n).unwrap()) {
                    if node.id > repr_node_max {
                        compatibility.excess += 1.0;
                    } else if repr
                        .nodes
                        .iter()
                        .map(|n| nodes.get(*n).unwrap())
                        .find(|n| n.id == node.id)
                        .is_none()
                    {
                        compatibility.disjoint += 1.0;
                    }
                }
                let n = genome.connections.len().max(repr.connections.len());
                compatibility.n = if n < 20 { 1.0 } else { n as f32 };
                compatibility.weight_difference /= num_weights;
                if compatibility.distance(&environment.compatibility_factors)
                    < environment.compatibility_threshold
                {
                    found = Some(s.id);
                    break;
                }
            }
            if let Some(f) = found {
                // add to existing
                // set genome species-id
            } else {
                // new
            }
        }
        // empty (remove species entity from list + despawn)
        // set this generation repr
    }
}
