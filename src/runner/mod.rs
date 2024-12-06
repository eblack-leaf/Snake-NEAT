use foliage::bevy_ecs;
use foliage::bevy_ecs::component::Component;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::prelude::{Event, Res, Trigger};
use foliage::bevy_ecs::system::{Query, ResMut, Resource};
use foliage::grid::Grid;
use foliage::leaf::Leaf;
use foliage::text::TextValue;
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
        todo!()
    }
    pub(crate) fn new(inputs: i32, outputs: i32) -> Self {
        let mut generator = 0;
        let mut existing = HashMap::new();
        // fully-connected innovations
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
        }
    }
}
#[derive(Component)]
pub(crate) struct Genome {
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
    pub(crate) ty: NodeType,
    pub(crate) value: f32,
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
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree) {
        todo!()
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
    pub(crate) fn distance(&self, factors: &CompatibilityFactors) -> f32 {
        todo!()
    }
}
#[derive(Resource, Copy, Clone)]
pub(crate) struct GameSpeed(pub(crate) i32);
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
