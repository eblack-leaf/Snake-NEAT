use crate::runner::game::{Game, Running};
use crate::runner::genome::{Evaluation, NetworkInput, NetworkOutput, Reward};
use environment::Environment;
use foliage::bevy_ecs;
use foliage::bevy_ecs::component::Component;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::prelude::{Event, Res, Trigger};
use foliage::bevy_ecs::system::{Query, ResMut, Resource};
use foliage::grid::Grid;
use foliage::interaction::OnClick;
use foliage::leaf::Leaf;
use foliage::text::TextValue;
use foliage::tree::Tree;
use game::GameSpeed;
use genome::Genome;
use innovation::ExistingInnovation;

mod compatibility;
mod connection;
pub(crate) mod environment;
mod game;
mod genome;
mod innovation;
mod node;
mod species;

#[derive(Event)]
pub(crate) struct RunnerIn {
    pub(crate) root: Entity,
}
#[derive(Resource)]
pub(crate) struct RunnerIds {
    pub(crate) root: Entity,
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
    pub(crate) expanded_view: Entity,
}
impl RunnerIn {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree, environment: Res<Environment>) {
        let mut environment = Environment::new();
        environment.population_count = 150;
        environment.input_size = 6;
        environment.output_size = 2;
        environment.compatibility_factors.c1 = 1.0;
        environment.compatibility_factors.c2 = 1.0;
        environment.compatibility_factors.c3 = 0.5;
        environment.compatibility_threshold = 3.0;
        let root = tree
            .spawn(Leaf::new().stem(Some(trigger.event().root)))
            .id();
        let gen = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let gen_text = tree.spawn(Leaf::new().stem(Some(gen)).elevation(0)).id();
        let gen_increment = tree.spawn(Leaf::new().stem(Some(gen)).elevation(0)).id();
        let gen_run_to = tree.spawn(Leaf::new().stem(Some(gen)).elevation(0)).id();
        let population_label = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let species_label = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let evaluate = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let process = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        let num_columns = 8;
        let num_rows = (environment.population_count / 8).max(1) as u32;
        let element_size = (300, 150);
        let view_size = (element_size.0 * num_columns, element_size.1 * num_rows);
        let grid = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(0))
            .insert(Grid::new(num_columns, num_rows).gap((4, 4)))
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
            run_to: false,
            best: None,
            species_id_gen: 0,
            genome_id_gen: 0,
            innovation: ExistingInnovation::new(environment.input_size, environment.output_size),
        };
        let game_grid = (60, 30);
        let reward = Reward::new(5.0, 1.75, 0.75);
        let expanded_view = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        // TODO elements of expanded-view (game, network, score-label, finished-signal, switch-view)
        for p in 0..environment.population_count {
            let view = tree.spawn(Leaf::new().stem(Some(grid))).id();
            let score_label = tree.spawn(Leaf::new().stem(Some(view))).id();
            let finished_signal = tree.spawn(Leaf::new().stem(Some(view))).id();
            let g = tree.spawn(Leaf::new().stem(Some(view)).elevation(0)).id();
            tree.entity(view).insert(GenomeView {
                score: score_label,
                finished_signal,
                genome: g,
            });
            let genome = Genome::new(
                &mut tree,
                g,
                environment.input_size,
                environment.output_size,
            );
            tree.entity(g).insert(genome);
            let snake = tree.spawn(Leaf::new().stem(Some(g))).id();
            let food = tree.spawn(Leaf::new().stem(Some(g))).id();
            let canvas = tree.spawn(Leaf::new().stem(Some(g))).id();
            let game = Game {
                snake,
                food,
                canvas,
                grid: game_grid,
                updated: false,
            };
            tree.entity(g)
                .insert(game)
                .insert(Running(false))
                .insert(Evaluation::default())
                .insert(NetworkInput::default())
                .insert(NetworkOutput::default())
                .insert(reward);
            runner.population.push(g);
            runner.genome_id_gen += 1;
        }
        tree.insert_resource(runner);
        tree.insert_resource(environment);
        let ids = RunnerIds {
            root,
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
            expanded_view,
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
#[derive(Resource)]
pub(crate) struct Runner {
    pub(crate) population: Vec<Entity>,
    pub(crate) next_gen: Vec<Genome>,
    pub(crate) species: Vec<Entity>,
    pub(crate) generation: Generation,
    pub(crate) requested_generation: Generation,
    pub(crate) run_to: bool,
    pub(crate) best: Option<Entity>,
    pub(crate) species_id_gen: SpeciesId,
    pub(crate) genome_id_gen: GenomeId,
    pub(crate) innovation: ExistingInnovation,
}
#[derive(Component, Copy, Clone)]
pub(crate) struct GenomeView {
    pub(crate) score: Entity,
    pub(crate) finished_signal: Entity,
    pub(crate) genome: Entity,
}
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
        runner.run_to = true;
        tree.trigger(Evaluate {});
    }
}
#[derive(Event)]
pub(crate) struct SelectGenome {}
impl SelectGenome {
    pub(crate) fn on_click(
        trigger: Trigger<OnClick>,
        mut tree: Tree,
        genome_views: Query<&GenomeView>,
    ) {
        let view = trigger.entity();
        let genome = genome_views.get(view).unwrap().genome;
        // copy genome to expanded-view.genome (deep copy not just clone component)
    }
}
#[derive(Event)]
pub(crate) struct Evaluate {}
impl Evaluate {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree, runner: ResMut<Runner>) {
        // run game instance to completion on each genome
        for genome in runner.population.iter().cloned() {
            tree.trigger_targets(EvaluateGenome {}, genome);
        }
    }
}
#[derive(Event)]
pub(crate) struct EvaluateGenome {}
impl EvaluateGenome {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree) {
        let genome = trigger.entity();
        // reset score
        tree.entity(genome).insert(Evaluation::default());
        // reset snake
        // reset food
        // start game
        tree.entity(genome).insert(Running(true));
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
