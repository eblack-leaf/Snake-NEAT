use foliage::bevy_ecs;
use foliage::bevy_ecs::component::Component;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::prelude::{Event, Res, Trigger};
use foliage::bevy_ecs::system::{Query, ResMut, Resource};
use foliage::grid::Grid;
use foliage::leaf::Leaf;
use foliage::text::{Text, TextValue};
use foliage::tree::Tree;
use std::collections::HashMap;

#[derive(Event)]
pub(crate) struct RunnerIn {
    pub(crate) root: Entity,
}
#[derive(Resource)]
pub(crate) struct RunnerIds {
    pub(crate) gen: Entity,
    pub(crate) gen_text: Entity,
    pub(crate) gen_increment: Entity,
    pub(crate) gen_run_to: Entity,
    pub(crate) population_label: Entity,
    pub(crate) species_label: Entity,
    pub(crate) evaluate: Entity,
    pub(crate) process: Entity,
    pub(crate) grid: Entity,
    pub(crate) game_speed: Entity,
    pub(crate) game_speed_decrement: Entity,
    pub(crate) game_speed_label: Entity,
    pub(crate) game_speed_increment: Entity,
}
impl RunnerIn {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree, environment: Res<Environment>) {
        let root = trigger.event().root;
        let gen = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let gen_text = tree.spawn(Leaf::new().stem(Some(gen)).elevation(0)).id();
        let gen_increment = tree.spawn(Leaf::new().stem(Some(gen)).elevation(0)).id();
        let gen_run_to = tree.spawn(Leaf::new().stem(Some(gen)).elevation(0)).id();
        let population_label = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let species_label = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let evaluate = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let process = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let x = 8;
        let y = (environment.population_count / 8).max(1) as u32;
        let grid = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(0))
            .insert(Grid::new(x, y).gap((2, 2)))
            .id();
        let game_speed = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let game_speed_decrement = tree
            .spawn(Leaf::new().stem(Some(game_speed)).elevation(0))
            .id();
        let game_speed_label = tree
            .spawn(Leaf::new().stem(Some(game_speed)).elevation(0))
            .id();
        let game_speed_increment = tree
            .spawn(Leaf::new().stem(Some(game_speed)).elevation(0))
            .id();
        tree.insert_resource(GameSpeed(1));
        let mut runner = Runner {
            population: vec![],
            next_gen: vec![],
            species: vec![],
            generation: 0,
            requested_generation: 1,
            best: None,
            species_id_gen: 0,
            genome_id_gen: 0,
            innovation: ExistingInnovation::new(environment.input_size, environment.output_size),
        };
        for p in 0..environment.population_count {
            let genome = Genome::new(&mut tree, environment.input_size, environment.output_size);
            let genome = tree
                .spawn(Leaf::new().stem(Some(grid)).elevation(0))
                .insert(genome)
                .id();
            // grid location
            runner.population.push(genome);
            runner.genome_id_gen += 1;
        }
        tree.insert_resource(runner);
        let ids = RunnerIds {
            gen,
            gen_text,
            gen_increment,
            gen_run_to,
            population_label,
            species_label,
            evaluate,
            process,
            grid,
            game_speed,
            game_speed_decrement,
            game_speed_label,
            game_speed_increment,
        };
        tree.insert_resource(ids);
    }
}
pub(crate) struct RunnerOut {}
impl RunnerOut {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        ids: Res<RunnerIds>,
        runner: Res<Runner>,
    ) {
        // despawn all ids
    }
}
pub(crate) type NodeId = i32;
pub(crate) type GenomeId = i32;
pub(crate) type SpeciesId = i32;
pub(crate) type Generation = i32;
pub(crate) type Innovation = i32;
pub(crate) type Activation = f32;
pub(crate) type Depth = i32;
pub(crate) type Fitness = f32;
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
    pub(crate) fn new(inputs: i32, outputs: i32) -> Self {
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
#[derive(Resource)]
pub(crate) struct Runner {
    pub(crate) population: Vec<Entity>,
    pub(crate) next_gen: Vec<Entity>,
    pub(crate) species: Vec<Entity>,
    pub(crate) generation: Generation,
    pub(crate) requested_generation: Generation,
    pub(crate) best: Option<Entity>,
    pub(crate) species_id_gen: SpeciesId,
    pub(crate) genome_id_gen: GenomeId,
    pub(crate) innovation: ExistingInnovation,
}
#[derive(Resource)]
pub(crate) struct Environment {
    pub(crate) population_count: i32,
    pub(crate) input_size: i32,
    pub(crate) output_size: i32,
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
#[derive(Component)]
pub(crate) struct Genome {
    pub(crate) id: GenomeId,
    pub(crate) nodes: Vec<Entity>,
    pub(crate) connections: Vec<Entity>,
    pub(crate) activation: Activation,
    pub(crate) depth: Depth,
    pub(crate) fitness: Fitness,
    pub(crate) species: SpeciesId,
}
impl Genome {
    pub(crate) fn new(tree: &mut Tree, input_size: i32, output_size: i32) -> Self {
        // setup nodes + connections
        todo!()
    }
}
#[derive(Copy, Clone)]
pub(crate) enum NodeType {
    Input,
    Output,
    Bias,
    Hidden,
}
#[derive(Component, Copy, Clone)]
pub(crate) struct Connection {
    pub(crate) weight: f32,
    pub(crate) innovation: Innovation,
    pub(crate) enabled: bool,
    pub(crate) from: Entity,
    pub(crate) to: Entity,
}
#[derive(Component, Copy, Clone)]
pub(crate) struct Node {
    pub(crate) id: NodeId,
    pub(crate) ty: NodeType,
    pub(crate) value: f32,
}
impl Node {
    pub(crate) fn new() -> Self {
        Self {
            id: 0,
            ty: NodeType::Hidden,
            value: 0.0,
        }
    }
}
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
pub(crate) struct CompatibilityFactors {
    pub(crate) c1: f32,
    pub(crate) c2: f32,
    pub(crate) c3: f32,
}
pub(crate) struct Compatibility {
    pub(crate) excess: f32,
    pub(crate) disjoint: f32,
    pub(crate) weight_difference: f32,
    pub(crate) n: f32,
}
impl Compatibility {
    pub(crate) fn new() -> Self {
        Self {
            excess: 0.0,
            disjoint: 0.0,
            weight_difference: 0.0,
            n: 0.0,
        }
    }
    pub(crate) fn distance(&self, factors: &CompatibilityFactors) -> f32 {
        todo!()
    }
}
#[derive(Resource, Copy, Clone)]
pub(crate) struct GameSpeed(pub(crate) i32);
#[derive(Event)]
pub(crate) struct UpdateSpeciesCountText {}
impl UpdateSpeciesCountText {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        ids: Res<RunnerIds>,
        mut text: Query<&mut TextValue>,
        runner: Res<Runner>,
    ) {
        text.get_mut(ids.species_label).unwrap().0 =
            format!("Num-Species: {}", runner.species.len());
    }
}

#[derive(Event)]
pub(crate) struct UpdateGenerationText {}
impl UpdateGenerationText {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        ids: Res<RunnerIds>,
        mut text: Query<&mut TextValue>,
        runner: ResMut<Runner>,
    ) {
        text.get_mut(ids.gen_text).unwrap().0 = format!(
            "Gen: {} -> {}",
            runner.generation, runner.requested_generation
        );
    }
}
#[derive(Event)]
pub(crate) struct IncrementGeneration {}
impl IncrementGeneration {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree, mut runner: ResMut<Runner>) {
        runner.requested_generation += 1;
        tree.trigger(UpdateGenerationText {});
    }
}
#[derive(Event)]
pub(crate) struct RunToGeneration {}
impl RunToGeneration {
    pub(crate) fn obs(trigger: Trigger<Self>, mut runner: ResMut<Runner>, mut tree: Tree) {
        for g in runner.generation..runner.requested_generation {
            // evaluate
            tree.trigger(Evaluate {});
            // process
            tree.trigger(Process {});
            // increment generation
            runner.generation += 1;
        }
        runner.requested_generation = runner.generation + 1;
        tree.trigger(UpdateGenerationText {});
    }
}
#[derive(Event)]
pub(crate) struct Evaluate {}
impl Evaluate {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree, runner: ResMut<Runner>) {
        // run game instance to completion on each genome
        for genome in runner.population.iter().cloned() {
            // reset score
            // start game
        }
    }
}
#[derive(Event)]
pub(crate) struct Process {}
impl Process {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree, runner: ResMut<Runner>) {
        // species %
        // cull
        // num_offspring
        // mutate & crossover (into runner.next_gen)
    }
}
