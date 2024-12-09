use crate::runner::compatibility::Compatibility;
use crate::runner::environment::Environment;
use crate::runner::genome::Genome;
use crate::runner::node::Node;
use crate::runner::{Fitness, Generation, Runner, SpeciesId, UpdateSpeciesCountText};
use foliage::bevy_ecs;
use foliage::bevy_ecs::change_detection::Res;
use foliage::bevy_ecs::component::Component;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::event::Event;
use foliage::bevy_ecs::observer::Trigger;
use foliage::bevy_ecs::prelude::Query;
use foliage::bevy_ecs::system::ResMut;
use foliage::tree::Tree;
use rand::Rng;

#[derive(Component, Clone)]
pub(crate) struct Species {
    pub(crate) id: SpeciesId,
    pub(crate) members: Vec<Entity>,
    pub(crate) last_improved: Generation,
    pub(crate) representative: Entity,
    pub(crate) repr_genome: Genome,
    pub(crate) max_fitness: Fitness,
    pub(crate) shared_fitness: Fitness,
    pub(crate) percent_total: f32,
}
impl Species {
    pub(crate) fn new(
        id: SpeciesId,
        representative: Entity,
        repr_genome: Genome,
        last_improved: Generation,
    ) -> Self {
        Self {
            id,
            members: vec![representative],
            last_improved,
            representative,
            repr_genome,
            max_fitness: 0.0,
            shared_fitness: 0.0,
            percent_total: 0.0,
        }
    }
}
#[derive(Event)]
pub(crate) struct Speciate {}

impl Speciate {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        mut runner: ResMut<Runner>,
        mut population: Query<&mut Genome>,
        environment: Res<Environment>,
    ) {
        for s in runner.species.iter_mut() {
            s.members.clear()
        }
        for p in runner.population.clone().iter().copied() {
            let mut found = None;
            for s in runner.species.iter().cloned() {
                let genome = population.get(p).unwrap();
                let repr = s.repr_genome;
                let mut compatibility = Compatibility::new();
                let repr_innovation_max = repr
                    .connections
                    .iter()
                    .map(|c| c.innovation)
                    .max()
                    .unwrap_or_default();
                let mut num_weights = 0.0;
                for conn in genome.connections.iter() {
                    let matching = repr
                        .connections
                        .iter()
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
                    .max_by(|a, b| a.id.partial_cmp(&b.id).unwrap())
                    .unwrap_or(&Node::new())
                    .id;
                for node in genome.nodes.iter() {
                    if node.id > repr_node_max {
                        compatibility.excess += 1.0;
                    } else if repr.nodes.iter().find(|n| n.id == node.id).is_none() {
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
                let idx = runner.species.iter().position(|s| s.id == f).unwrap();
                runner.species.get_mut(idx).unwrap().members.push(p);
                // set genome species-id
                population.get_mut(p).unwrap().species = f;
            } else {
                // new
                let id = runner.species_id_gen;
                runner.species_id_gen += 1;
                let gen = runner.generation;
                runner
                    .species
                    .push(Species::new(id, p, population.get(p).unwrap().clone(), gen));
            }
        }
        let mut empty = vec![];
        for species in runner.species.iter_mut() {
            if !species.members.is_empty() {
                let rand_idx = rand::thread_rng().gen_range(0..species.members.len());
                let representative = *species.members.get(rand_idx).unwrap();
                species.representative = representative;
                species.repr_genome = population.get(species.representative).unwrap().clone();
            } else {
                empty.push(species.id);
            }
        }
        for id in empty.iter_mut() {
            *id = runner.species.iter().position(|s| s.id == *id).unwrap();
        }
        empty.sort();
        empty.reverse();
        for idx in empty {
            runner.species.remove(idx);
        }
        tree.trigger(UpdateSpeciesCountText {});
    }
}
