use crate::runner::connection::Connection;
use crate::runner::environment::Environment;
use crate::runner::node::{Node, NodeType};
use crate::runner::{Depth, Fitness, GenomeId, NodeId, SpeciesId};
use foliage::bevy_ecs;
use foliage::bevy_ecs::component::Component;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::event::Event;
use foliage::bevy_ecs::prelude::{Query, Res, Trigger};
use foliage::tree::Tree;
use rand::Rng;

#[derive(Component)]
pub(crate) struct Genome {
    pub(crate) id: GenomeId,
    pub(crate) nodes: Vec<Node>,
    pub(crate) connections: Vec<Connection>,
    pub(crate) depth: Depth,
    pub(crate) fitness: Fitness,
    pub(crate) species: SpeciesId,
    pub(crate) node_id_gen: NodeId,
}
impl Genome {
    pub(crate) fn new(
        tree: &mut Tree,
        id: GenomeId,
        this: Entity,
        input_size: usize,
        output_size: usize,
    ) -> Self {
        // setup nodes + connections
        if input_size <= 0 || output_size <= 0 {
            panic!("Genome has no input/output dimensions");
        }
        let mut nodes = Vec::new();
        let mut connections = Vec::new();
        for input in 0..input_size {
            nodes.push(Node::explicit(input, NodeType::Input));
        }
        for output in input_size..input_size + output_size {
            nodes.push(Node::explicit(output, NodeType::Output));
        }
        let node_id_gen = nodes.len();
        let mut innovation = 0;
        for i in 0..input_size {
            for o in input_size..input_size + output_size {
                let connection =
                    Connection::new(i, o, rand::thread_rng().gen_range(0.0..1.0), innovation);
                connections.push(connection);
                innovation += 1;
            }
        }
        for bias in input_size + output_size..input_size + output_size * 2 {
            for o in input_size..input_size + output_size {
                let connection =
                    Connection::new(bias, o, rand::thread_rng().gen_range(0.0..1.0), innovation);
                connections.push(connection);
                innovation += 1;
            }
        }
        Self {
            id,
            nodes,
            connections,
            depth: 1,
            fitness: 0.0,
            species: 0,
            node_id_gen,
        }
    }
}
#[derive(Component, Copy, Clone)]
pub(crate) struct Reward {
    pub(crate) can_move_towards_food: bool,
    pub(crate) moved_towards_food: bool,
    pub(crate) collected_food: bool,
    pub(crate) food_collection_reward: Fitness,
    pub(crate) towards_food_reward: Fitness,
    pub(crate) can_move_towards_food_reward: Fitness,
}
impl Reward {
    pub(crate) fn new(fc: Fitness, tf: Fitness, cmtf: Fitness) -> Self {
        Self {
            can_move_towards_food: false,
            moved_towards_food: false,
            collected_food: false,
            food_collection_reward: fc,
            towards_food_reward: tf,
            can_move_towards_food_reward: cmtf,
        }
    }
}
#[derive(Component, Copy, Clone, Default)]
pub(crate) struct Evaluation {
    pub(crate) fitness: Fitness,
    pub(crate) total_food_collected: i32,
    pub(crate) num_turns_taken: i32,
}
#[derive(Component, Copy, Clone, Default)]
pub(crate) struct NetworkInput {
    pub(crate) can_move_left: bool,
    pub(crate) can_move_right: bool,
    pub(crate) can_move_forward: bool,
    pub(crate) is_food_left: bool,
    pub(crate) is_food_right: bool,
    pub(crate) is_food_forward: bool,
}
impl NetworkInput {
    pub(crate) fn get_channel(&self, i: usize) -> f32 {
        match i {
            0 => f32::from(self.can_move_left),
            1 => f32::from(self.can_move_right),
            2 => f32::from(self.can_move_forward),
            3 => f32::from(self.is_food_left),
            4 => f32::from(self.is_food_right),
            5 => f32::from(self.is_food_forward),
            _ => panic!("no-channel"),
        }
    }
}
#[derive(Component, Default)]
pub(crate) struct NetworkOutput {
    pub(crate) move_left: bool,
    pub(crate) move_right: bool,
}
#[derive(Component, Clone, Default)]
pub(crate) struct Activations {
    pub(crate) values: Vec<f32>,
}
impl Activations {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            values: vec![0.0; size],
        }
    }
}
#[derive(Event)]
pub(crate) struct Activate {}
impl Activate {
    pub(crate) const ACTIVATION_SCALE: f32 = 4.9;
    pub(crate) fn sigmoid(z: f32) -> f32 {
        1.0 / (1.0 + (-z).exp())
    }
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        inputs: Query<&NetworkInput>,
        mut outputs: Query<&mut NetworkOutput>,
        mut storage: Query<&mut Activations>,
        genomes: Query<&Genome>,
        environment: Res<Environment>,
    ) {
        let genome = genomes.get(trigger.entity()).unwrap();
        let input = inputs.get(trigger.entity()).unwrap();
        let mut summations = vec![0.0; genome.nodes.len()];
        let mut activations = vec![0.0; genome.nodes.len()];
        for _relax in 0..genome.depth {
            let mut solved = vec![false; environment.output_size];
            let mut valid = vec![false; genome.nodes.len()];
            for i in 0..environment.input_size as usize {
                *activations.get_mut(i).unwrap() = input.get_channel(i);
                *summations.get_mut(i).unwrap() = input.get_channel(i);
                *valid.get_mut(i).unwrap() = true;
            }
            for bias in (environment.input_size + environment.output_size)
                ..(environment.input_size + environment.output_size * 2)
            {
                let bias = bias;
                *activations.get_mut(bias).unwrap() = 1.0;
                *summations.get_mut(bias).unwrap() = 1.0;
                *valid.get_mut(bias).unwrap() = true;
            }
            const ABORT: usize = 20;
            let mut abort = 0;
            let non_input = genome
                .nodes
                .iter()
                .filter(|n| n.ty != NodeType::Input)
                .copied()
                .collect::<Vec<_>>();
            while solved.iter().any(|s| *s == false) && abort < ABORT {
                if abort == ABORT {
                    return;
                }
                for non in non_input.iter() {
                    *summations.get_mut(non.id).unwrap() = 0.0;
                    *valid.get_mut(non.id).unwrap() = false;
                    let incoming = genome
                        .connections
                        .iter()
                        .filter(|c| c.to == non.id)
                        .cloned()
                        .collect::<Vec<_>>();
                    let current_values = incoming
                        .iter()
                        .map(|c| activations.get(c.from).copied().unwrap_or_default())
                        .collect::<Vec<_>>();
                    let sum = current_values
                        .iter()
                        .enumerate()
                        .map(|(i, a)| *a * incoming.get(i).unwrap().weight)
                        .sum::<f32>();
                    *summations.get_mut(non.id).unwrap() += sum;
                    if valid.iter().any(|a| *a == true) {
                        *valid.get_mut(non.id).unwrap() = true;
                    }
                }
                for non in non_input.iter() {
                    if *valid.get(non.id).unwrap() {
                        let out = summations.get(non.id).copied().unwrap();
                        *activations.get_mut(non.id).unwrap() =
                            Self::sigmoid(Self::ACTIVATION_SCALE * out);
                        for output_test in
                            environment.input_size..environment.input_size + environment.output_size
                        {
                            if output_test == non.id {
                                solved[output_test - environment.input_size] = true;
                            }
                        }
                    }
                }
                abort += 1;
            }
        }
        let mut output = outputs.get_mut(trigger.entity()).unwrap();
        for out in environment.input_size..environment.input_size + environment.output_size {
            match out - environment.input_size {
                0 => output.move_left = *activations.get(0).unwrap() >= 0.5,
                1 => output.move_right = *activations.get(1).unwrap() >= 0.5,
                _ => panic!("no-channel"),
            }
        }
        storage.get_mut(trigger.entity()).unwrap().values = activations;
    }
}
