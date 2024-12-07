use crate::runner::compatibility::CompatibilityFactors;
use crate::runner::connection::Connection;
use crate::runner::genome::Genome;
use crate::runner::innovation::ExistingInnovation;
use crate::runner::node::{Node, NodeType};
use crate::runner::{Generation, GenomeId};
use foliage::bevy_ecs;
use foliage::bevy_ecs::prelude::Resource;
use rand::Rng;

#[derive(Resource)]
pub(crate) struct Environment {
    pub(crate) population_count: i32,
    pub(crate) input_size: usize,
    pub(crate) output_size: usize,
    pub(crate) compatibility_factors: CompatibilityFactors,
    pub(crate) compatibility_threshold: f32,
    pub(crate) stagnation_threshold: Generation,
    pub(crate) only_mutate: f32,
    pub(crate) elitism: f32,
    pub(crate) crossover_only: f32,
    pub(crate) inherit_disable: f32,
    pub(crate) add_connection: f32,
    pub(crate) connection_weight: f32,
    pub(crate) perturb: f32,
    pub(crate) add_node: f32,
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
            stagnation_threshold: 0,
            only_mutate: 0.0,
            elitism: 0.0,
            crossover_only: 0.0,
            inherit_disable: 0.0,
            add_connection: 0.0,
            connection_weight: 0.0,
            perturb: 0.0,
            add_node: 0.0,
        }
    }
    pub(crate) fn mutate(
        &self,
        mut genome: Genome,
        existing_innovation: &mut ExistingInnovation,
    ) -> Genome {
        for conn in genome.connections.iter_mut() {
            if rand::thread_rng().gen_range(0.0..1.0) < self.connection_weight {
                if rand::thread_rng().gen_range(0.0..1.0) < self.perturb {
                    let perturb = rand::thread_rng().gen_range(-1.0..1.0);
                    conn.weight += perturb;
                } else {
                    conn.weight = rand::thread_rng().gen_range(0.0..1.0);
                }
            }
        }
        if rand::thread_rng().gen_range(0.0..1.0) < self.add_node {
            if genome.connections.is_empty() {
                return genome;
            }
            let new = Node::explicit(genome.node_id_gen, NodeType::Hidden);
            // println!("adding node {}", new.id);
            genome.node_id_gen += 1;
            let idx = rand::thread_rng().gen_range(0..genome.connections.len());
            let existing_connection = genome.connections.get(idx).cloned().unwrap();
            genome.connections.get_mut(idx).unwrap().enabled = false;
            let a = Connection::new(
                existing_connection.from,
                new.id,
                1.0,
                existing_innovation.check(existing_connection.from, new.id),
            );
            let b = Connection::new(
                new.id,
                existing_connection.to,
                existing_connection.weight,
                existing_innovation.check(new.id, existing_connection.to),
            );
            genome.connections.push(a);
            genome.connections.push(b);
            genome.nodes.push(new);
        } else if rand::thread_rng().gen_range(0.0..1.0) < self.add_connection {
            if let Some((input, output)) = self.select_connection_nodes(&genome) {
                // println!(
                //     "adding connection: from: {}:{:?} to: {}:{:?}",
                //     input.id, input.ty, output.id, output.ty
                // );
                let connection = Connection::new(
                    input.id,
                    output.id,
                    rand::thread_rng().gen_range(0.0..1.0),
                    existing_innovation.check(input.id, output.id),
                );
                genome.connections.push(connection);
            }
        }
        genome
    }
    pub(crate) fn crossover(&self, id: GenomeId, best: Genome, other: Genome) -> Genome {
        let mut child = Genome::new(id, self.input_size, self.output_size);
        for conn in best.connections.iter() {
            let mut gene = conn.clone();
            let mut from_type = best.nodes.iter().find(|n| n.id == gene.from).unwrap().ty;
            let mut to_type = best.nodes.iter().find(|n| n.id == gene.to).unwrap().ty;
            if let Some(matching) = other
                .connections
                .iter()
                .find(|c| c.innovation == conn.innovation)
            {
                if rand::thread_rng().gen_range(0.0..1.0) < 0.5 {
                    gene = matching.clone();
                    from_type = other.nodes.iter().find(|n| n.id == gene.from).unwrap().ty;
                    to_type = other.nodes.iter().find(|n| n.id == gene.to).unwrap().ty;
                }
                if !conn.enabled || !matching.enabled {
                    if rand::thread_rng().gen_range(0.0..1.0) < self.inherit_disable {
                        gene.enabled = false;
                    }
                }
            }
            if child.nodes.iter().find(|n| n.id == gene.from).is_none() {
                let n = Node::explicit(gene.from, from_type);
                child.nodes.push(n);
            }
            if child.nodes.iter().find(|n| n.id == gene.to).is_none() {
                let n = Node::explicit(gene.to, to_type);
                child.nodes.push(n);
            }
            if child
                .connections
                .iter()
                .find(|c| c.from == gene.from && c.to == gene.to)
                .is_none()
            {
                child.connections.push(gene);
            }
        }
        child.node_id_gen = child.nodes.len();
        child
    }
    pub(crate) fn select_connection_nodes(&self, genome: &Genome) -> Option<(Node, Node)> {
        let potential_inputs = genome
            .nodes
            .iter()
            .filter(|n| n.ty != NodeType::Output)
            .copied()
            .collect::<Vec<_>>();
        let potential_outputs = genome
            .nodes
            .iter()
            .filter(|n| n.ty != NodeType::Input && n.ty != NodeType::Bias)
            .copied()
            .collect::<Vec<_>>();
        if potential_inputs.is_empty() || potential_outputs.is_empty() {
            return None;
        }
        let idx = rand::thread_rng().gen_range(0..potential_inputs.len());
        let mut input = potential_inputs.get(idx).copied().unwrap();
        let idx = rand::thread_rng().gen_range(0..potential_outputs.len());
        let mut output = potential_outputs.get(idx).copied().unwrap();
        while input.id == output.id && potential_inputs.len() > 1 {
            let idx = rand::thread_rng().gen_range(0..potential_inputs.len());
            input = potential_inputs.get(idx).copied().unwrap();
        }
        while input.id == output.id && potential_outputs.len() > 1 {
            let idx = rand::thread_rng().gen_range(0..potential_outputs.len());
            output = potential_outputs.get(idx).copied().unwrap();
        }
        if input.id == output.id {
            return None;
        }
        if genome
            .connections
            .iter()
            .find(|c| c.from == input.id && c.to == output.id)
            .is_some()
        {
            // recursive create_connection if can
            // if potential_inputs.len() > 1 || potential_outputs.len() > 1 {
            //     return self.select_connection_nodes(&genome);
            // }
            return None;
        }
        Some((input, output))
    }
}
