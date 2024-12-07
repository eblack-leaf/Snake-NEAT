use crate::runner::game::{Game, Running};
use crate::runner::genome::{Evaluation, MaxDepthCheck, NetworkInput, NetworkOutput, Reward};
use crate::runner::species::{Speciate, Species};
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
use rand::Rng;

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
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree) {
        let mut environment = Environment::new();
        environment.population_count = 150;
        environment.input_size = 6;
        environment.output_size = 2;
        environment.compatibility_factors.c1 = 1.0;
        environment.compatibility_factors.c2 = 1.0;
        environment.compatibility_factors.c3 = 0.4;
        environment.compatibility_threshold = 3.0;
        environment.stagnation_threshold = 15;
        environment.elitism = 0.2;
        environment.add_connection = 0.2;
        environment.add_node = 0.07;
        environment.inherit_disable = 0.75;
        environment.only_mutate = 0.25;
        environment.crossover_only = 0.2;
        environment.connection_weight = 0.8;
        environment.perturb = 0.9;
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
        tree.insert_resource(GameSpeed::new(1));
        let mut runner = Runner {
            population: vec![],
            species: vec![],
            generation: 0,
            requested_generation: 1,
            run_to: false,
            best: None,
            species_id_gen: 0,
            genome_id_gen: 0,
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
                runner.genome_id_gen,
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
        runner.best.replace((
            Genome::new(0, environment.input_size, environment.output_size),
            Evaluation::default(),
        ));
        tree.insert_resource(ExistingInnovation::new(
            environment.input_size,
            environment.output_size,
        ));
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
        tree.entity(ids.root).despawn();
    }
}
pub(crate) type NodeId = usize;
pub(crate) type GenomeId = usize;
pub(crate) type SpeciesId = usize;
pub(crate) type Generation = i32;
pub(crate) type Innovation = i32;
pub(crate) type Depth = i32;
pub(crate) type Fitness = f32;
#[derive(Resource)]
pub(crate) struct Runner {
    pub(crate) population: Vec<Entity>,
    pub(crate) species: Vec<Species>,
    pub(crate) generation: Generation,
    pub(crate) requested_generation: Generation,
    pub(crate) run_to: bool,
    pub(crate) best: Option<(Genome, Evaluation)>,
    pub(crate) species_id_gen: SpeciesId,
    pub(crate) genome_id_gen: GenomeId,
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
        // copy genome to expanded-view.genome
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
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        mut runner: ResMut<Runner>,
        mut existing_innovation: ResMut<ExistingInnovation>,
        evaluations: Query<(Entity, &Evaluation)>,
        environment: Res<Environment>,
        genomes: Query<&Genome>,
    ) {
        // species %
        let mut to_cull = vec![];
        let gen = runner.generation;
        for species in runner.species.iter_mut() {
            let max = species
                .members
                .iter()
                .map(|e| evaluations.get(*e).unwrap().1.fitness)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            if max > species.max_fitness {
                species.max_fitness = max;
                species.last_improved = gen;
            }

            if gen > species.last_improved + environment.stagnation_threshold {
                to_cull.push(species.id);
            }
        }
        // cull
        for id in to_cull.iter_mut() {
            *id = runner.species.iter().position(|s| s.id == *id).unwrap();
        }
        to_cull.sort();
        to_cull.reverse();
        for idx in to_cull {
            if runner.species.len() == 1 {
                // replace all with starting genomes
                // put all into starting species
                // for now continuing
                continue;
            }
            runner.species.remove(idx);
        }
        let min_fitness = evaluations
            .iter()
            .map(|a| (a.0, *a.1))
            .min_by(|a, b| a.1.fitness.partial_cmp(&b.1.fitness).unwrap())
            .unwrap()
            .1
            .fitness;
        let current_best = evaluations
            .iter()
            .map(|a| (a.0, *a.1))
            .max_by(|a, b| a.1.fitness.partial_cmp(&b.1.fitness).unwrap())
            .unwrap();
        if current_best.1.fitness > runner.best.as_ref().unwrap().1.fitness {
            runner
                .best
                .replace((genomes.get(current_best.0).unwrap().clone(), current_best.1));
        }
        let fitness_range = (current_best.1.fitness - min_fitness).max(1.0);
        for species in runner.species.iter_mut() {
            species.shared_fitness = 0.0;
            if species.members.is_empty() {
                continue;
            }
            for e in species.members.iter() {
                let eval = *evaluations.get(*e).unwrap().1;
                species.shared_fitness += eval.fitness;
            }
            if species.shared_fitness <= 0.0 {
                continue;
            }
            species.shared_fitness /= species.members.len() as f32;
        }
        let total_fitness = runner.species.iter().map(|s| s.shared_fitness).sum::<f32>();
        for species in runner.species.iter_mut() {
            species.percent_total = species.shared_fitness / total_fitness;
        }
        let mut next_gen = vec![];
        let mut remaining = environment.population_count as usize;
        let mut next_gen_id = 0;
        for species in runner.species.iter_mut() {
            let mut offspring_count =
                (species.percent_total * environment.population_count as f32).floor();
            remaining -= offspring_count as usize;
            if remaining <= 0 {
                offspring_count += remaining as f32;
            }
            let only_mutate = (offspring_count * environment.only_mutate).floor();
            let to_crossover = offspring_count - only_mutate;
            let mut members = species
                .members
                .iter()
                .map(|m| evaluations.get(*m).unwrap())
                .map(|e| (e.0, *e.1))
                .collect::<Vec<_>>();
            members.sort_by(|a, b| a.1.fitness.partial_cmp(&b.1.fitness).unwrap());
            members.reverse();
            let elite_bound = ((environment.elitism * members.len() as f32) as usize)
                .min(members.len())
                .max(1);
            let elites = members.get(0..elite_bound).unwrap().to_vec();
            for _om in 0..only_mutate as usize {
                let selected = elites
                    .get(rand::thread_rng().gen_range(0..elites.len()))
                    .copied()
                    .unwrap();
                let mut mutated = environment.mutate(
                    genomes.get(selected.0).unwrap().clone(),
                    &mut existing_innovation,
                );
                mutated.id = next_gen_id;
                next_gen_id += 1;
                next_gen.push(mutated);
            }
            for _c in 0..to_crossover as usize {
                let parent1 = elites
                    .get(rand::thread_rng().gen_range(0..elites.len()))
                    .cloned()
                    .unwrap();
                let parent1_genome = genomes.get(parent1.0).unwrap().clone();
                let mut parent2 = elites
                    .get(rand::thread_rng().gen_range(0..elites.len()))
                    .cloned()
                    .unwrap();
                let mut parent2_genome = genomes.get(parent2.0).unwrap().clone();
                while parent1_genome.id == parent2_genome.id && elites.len() > 1 {
                    parent2 = elites
                        .get(rand::thread_rng().gen_range(0..elites.len()))
                        .cloned()
                        .unwrap();
                    parent2_genome = genomes.get(parent2.0).cloned().unwrap();
                }
                let (best, other) = if parent1.1.fitness > parent2.1.fitness {
                    (parent1_genome, parent2_genome)
                } else if parent2.1.fitness > parent1.1.fitness {
                    (parent2_genome, parent1_genome)
                } else {
                    if rand::thread_rng().gen_range(0.0..1.0) < 0.5 {
                        (parent2_genome, parent1_genome)
                    } else {
                        (parent1_genome, parent2_genome)
                    }
                };
                let crossover = environment.crossover(next_gen_id, best, other);
                next_gen_id += 1;
                let crossover =
                    if rand::thread_rng().gen_range(0.0..1.0) < environment.crossover_only {
                        crossover
                    } else {
                        environment.mutate(crossover, &mut existing_innovation)
                    };
                next_gen.push(crossover);
            }
        }
        for (i, next) in next_gen.drain(..).enumerate() {
            tree.entity(*runner.population.get(i).unwrap()).insert(next);
        }
        // max-depth
        tree.trigger(MaxDepthCheck {});
        // speciate
        tree.trigger(Speciate {});
    }
}
