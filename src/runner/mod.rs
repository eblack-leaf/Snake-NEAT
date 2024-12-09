use crate::overview::{IconHandles, SECTION_OUT_END, SIDE_PANEL_WIDTH, VIEW_AREA};
use crate::runner::game::{Game, GameGrid, Running};
use crate::runner::genome::{
    Activations, Evaluation, MaxDepthCheck, NetworkInput, NetworkOutput, Reward,
};
use crate::runner::species::{Speciate, Species};
use environment::Environment;
use foliage::anim::Animation;
use foliage::bevy_ecs::component::Component;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::prelude::{Event, IntoSystemConfigs, Res, Trigger};
use foliage::bevy_ecs::system::{Query, ResMut, Resource};
use foliage::color::{Blue, Grey, Monochromatic, Orange};
use foliage::coordinate::section::Section;
use foliage::coordinate::LogicalContext;
use foliage::elm::{Elm, ExternalStage};
use foliage::grid::aspect::stem;
use foliage::grid::responsive::evaluate::{ScrollContext, Scrollable};
use foliage::grid::responsive::ResponsiveLocation;
use foliage::grid::unit::TokenUnit;
use foliage::grid::Grid;
use foliage::interaction::OnClick;
use foliage::leaf::{EvaluateCore, Leaf};
use foliage::opacity::Opacity;
use foliage::panel::{Panel, Rounding};
use foliage::style::Coloring;
use foliage::text::{FontSize, Text, TextValue};
use foliage::tree::{EcsExtension, Tree};
use foliage::twig::button::Button;
use foliage::{bevy_ecs, Root};
use game::GameSpeed;
use genome::Genome;
use innovation::ExistingInnovation;
use rand::Rng;

mod compatibility;
mod connection;
pub(crate) mod environment;
pub(crate) mod game;
pub(crate) mod genome;
mod innovation;
mod node;
pub(crate) mod species;
impl Root for Runner {
    fn attach(elm: &mut Elm) {
        elm.scheduler
            .main
            .add_systems(game::run.in_set(ExternalStage::Action));
    }
}
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
    pub(crate) grid_wrapper: Entity,
    pub(crate) grid: Entity,
    pub(crate) game_speed: Entity,
    pub(crate) game_speed_decrement: Entity,
    pub(crate) game_speed_label: Entity,
    pub(crate) game_speed_increment: Entity,
    pub(crate) expanded_view: Entity,
    pub(crate) gen_stop: Entity,
    pub(crate) num_running: Entity,
}
impl RunnerIn {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        sections: Query<&Section<LogicalContext>>,
    ) {
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
        environment.max_turns = 500;
        tree.start_sequence(|seq| {
            seq.animate(
                Animation::new(Opacity::new(1.0))
                    .start(SECTION_OUT_END + 100)
                    .end(SECTION_OUT_END + 400)
                    .targeting(trigger.event().root),
            );
        });
        let root = tree
            .spawn(Leaf::new().stem(Some(trigger.event().root)).elevation(-1))
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .right(stem().right())
                    .top(stem().top())
                    .bottom(stem().bottom()),
            )
            .insert(EvaluateCore::recursive())
            .id();
        let side = 200;
        let button_size = 32;
        let gen = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(-1))
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .width(side.px())
                    .top(stem().top())
                    .height(20.percent().height().of(stem())),
            )
            .insert(EvaluateCore::recursive())
            .id();
        let gen_text = tree
            .spawn(Leaf::new().stem(Some(gen)).elevation(0))
            .insert(Text::new("Gen: 0 -> 1", FontSize::new(14), Grey::plus_two()).centered())
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .width(side.px())
                    .top(stem().top())
                    .bottom(50.percent().height().from(stem())),
            )
            .insert(EvaluateCore::recursive())
            .id();
        let gen_increment = tree
            .spawn(Leaf::new().stem(Some(gen)).elevation(0))
            .insert(
                Button::new(
                    IconHandles::Check,
                    Coloring::new(Grey::plus_two(), Grey::minus_two()),
                )
                .circle(),
            )
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .width(button_size.px())
                    .top(50.percent().height().from(stem()) + 8.px())
                    .height(button_size.px()),
            )
            .observe(IncrementGeneration::obs)
            .insert(EvaluateCore::recursive())
            .id();
        let gen_run_to = tree
            .spawn(Leaf::new().stem(Some(gen)).elevation(0))
            .insert(
                Button::new(
                    IconHandles::Check,
                    Coloring::new(Grey::plus_two(), Grey::minus_two()),
                )
                .circle(),
            )
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left() + button_size.px() + 8.px())
                    .width(button_size.px())
                    .top(50.percent().height().from(stem()) + 8.px())
                    .height(button_size.px()),
            )
            .observe(RunToGeneration::obs)
            .insert(EvaluateCore::recursive())
            .id();
        let gen_stop = tree
            .spawn(Leaf::new().stem(Some(gen)).elevation(0))
            .insert(
                Button::new(
                    IconHandles::Check,
                    Coloring::new(Grey::plus_two(), Grey::minus_two()),
                )
                .circle(),
            )
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left() + (button_size * 2).px() + 16.px())
                    .width(button_size.px())
                    .top(50.percent().height().from(stem()) + 8.px())
                    .height(button_size.px()),
            )
            .observe(StopGeneration::obs)
            .insert(EvaluateCore::recursive())
            .id();
        let population_label = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(-1))
            .insert(
                Text::new(
                    format!("Population: {}", environment.population_count),
                    FontSize::new(14),
                    Grey::plus_two(),
                )
                .centered(),
            )
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .width(side.px())
                    .top(20.percent().height().from(stem()))
                    .height((20 / 3).percent().height().of(stem())),
            )
            .insert(EvaluateCore::recursive())
            .id();
        let species_label = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(-1))
            .insert(Text::new("Species: 0", FontSize::new(14), Grey::plus_two()).centered())
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .width(side.px())
                    .top((20 + 20 / 3).percent().height().from(stem()))
                    .height((20 / 3).percent().height().of(stem())),
            )
            .insert(EvaluateCore::recursive())
            .id();
        let num_running = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(0))
            .insert(Text::new("Running: 0", FontSize::new(14), Grey::plus_two()).centered())
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .width(side.px())
                    .top((20 + 40 / 3).percent().height().from(stem()))
                    .height((20 / 3).percent().height().of(stem())),
            )
            .insert(EvaluateCore::recursive())
            .id();
        let evaluate = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(-1))
            .insert(
                Button::new(
                    IconHandles::Check,
                    Coloring::new(Grey::minus_two(), Grey::plus_two()),
                )
                .with_text("evaluate", FontSize::new(14)),
            )
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left() + 8.px())
                    .width(side.px() - 16.px())
                    .height(7.percent().height().of(stem()))
                    .top(40.percent().height().from(stem())),
            )
            .observe(EvaluateWrapper::obs)
            .insert(EvaluateCore::recursive())
            .id();
        let process = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(-1))
            .insert(
                Button::new(
                    IconHandles::Check,
                    Coloring::new(Grey::minus_two(), Grey::plus_two()),
                )
                .with_text("process", FontSize::new(14)),
            )
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left() + 8.px())
                    .width(side.px() - 16.px())
                    .height(7.percent().height().of(stem()))
                    .top(50.percent().height().from(stem())),
            )
            .observe(ProcessWrapper::obs)
            .insert(EvaluateCore::recursive())
            .id();
        let game_speed = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(-1))
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .width(side.px())
                    .top(60.percent().height().from(stem()))
                    .height(20.percent().height().of(stem())),
            )
            .insert(EvaluateCore::recursive())
            .id();
        let game_speed_decrement = tree
            .spawn(Leaf::new().stem(Some(game_speed)).elevation(0))
            .insert(
                Button::new(
                    IconHandles::Check,
                    Coloring::new(Grey::plus_two(), Grey::minus_two()),
                )
                .circle(),
            )
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left() + 8.px())
                    .width(button_size.px())
                    .top(50.percent().height().from(stem()))
                    .height(button_size.px()),
            )
            .observe(GameSpeedChange::decrement)
            .insert(EvaluateCore::recursive())
            .id();
        let game_speed_label = tree
            .spawn(Leaf::new().stem(Some(game_speed)).elevation(0))
            .insert(Text::new("Speed: 1", FontSize::new(14), Grey::plus_two()).centered())
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .right(stem().right())
                    .top(stem().top())
                    .bottom(50.percent().height().from(stem())),
            )
            .insert(EvaluateCore::recursive())
            .id();
        let game_speed_increment = tree
            .spawn(Leaf::new().stem(Some(game_speed)).elevation(0))
            .insert(
                Button::new(
                    IconHandles::Check,
                    Coloring::new(Grey::plus_two(), Grey::minus_two()),
                )
                .circle(),
            )
            .insert(
                ResponsiveLocation::new()
                    .right(stem().right() - 8.px())
                    .width(button_size.px())
                    .top(50.percent().height().from(stem()))
                    .height(button_size.px()),
            )
            .observe(GameSpeedChange::increment)
            .insert(EvaluateCore::recursive())
            .id();
        tree.insert_resource(GameSpeed::new(1));
        let game_grid = GameGrid::new(60, 30);
        let mut runner = Runner {
            population: vec![],
            species: vec![],
            generation: 0,
            requested_generation: 1,
            run_to: false,
            best: None,
            species_id_gen: 0,
            genome_id_gen: 0,
            finished: environment.population_count,
            game_grid,
            canvas_size: (0, 0),
        };
        let main = VIEW_AREA.0 as i32 - SIDE_PANEL_WIDTH as i32 - side;
        let element_label = 20;
        let element_size = (200, 100 + element_label);
        let num_columns = main / (element_size.0 + 16);
        let num_rows = (environment.population_count / num_columns).max(1) + 1;
        let view_size = (num_columns * element_size.0, num_rows * element_size.1);
        let canvas_size = (element_size.0, element_size.1 - element_label);
        runner.canvas_size = canvas_size;
        let grid_wrapper = tree
            .spawn(Leaf::new().stem(Some(root)).elevation(0))
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left() + side.px())
                    .width(main.px())
                    .top(stem().top())
                    .bottom(stem().bottom()),
            )
            .insert(Scrollable::new())
            .insert(EvaluateCore::recursive())
            .id();
        let grid = tree
            .spawn(Leaf::new().stem(Some(grid_wrapper)).elevation(0))
            .insert(Grid::new(num_columns as u32, num_rows as u32).gap((8, 4)))
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .width(view_size.0.px())
                    .height(view_size.1.px())
                    .top(stem().top()),
            )
            .insert(ScrollContext::new(grid_wrapper))
            .insert(EvaluateCore::recursive())
            .id();
        let reward = Reward::new(5.0, 1.75, 0.75);
        let mut locations = vec![];
        for c in 0..num_columns {
            for r in 0..num_rows {
                let c = c + 1;
                let r = r + 1;
                locations.push(
                    ResponsiveLocation::new()
                        .left(c.column().begin().of(stem()))
                        .width(element_size.0.px())
                        .top(r.row().begin().of(stem()))
                        .height(element_size.1.px()),
                );
            }
        }
        for p in 0..environment.population_count {
            let view = tree
                .spawn(Leaf::new().stem(Some(grid)).elevation(-1))
                .insert(locations.get(p as usize).unwrap().clone())
                // .insert(ScrollContext::new(grid_wrapper))
                .insert(EvaluateCore::recursive())
                .id(); // panel for game-card
            let score_label = tree
                .spawn(Leaf::new().stem(Some(view)).elevation(-1))
                .insert(Text::new("Score: 0", FontSize::new(12), Grey::plus_two()))
                .insert(
                    ResponsiveLocation::new()
                        .left(stem().left())
                        .width(100.px())
                        .height(element_label.px())
                        .bottom(stem().bottom()),
                )
                // .insert(ScrollContext::new(grid_wrapper))
                .insert(EvaluateCore::recursive())
                .id(); // text
            let finished_signal = tree
                .spawn(Leaf::new().stem(Some(view)).elevation(-1))
                .insert(Panel::new(Rounding::all(1.0), Orange::base()))
                // .insert(ScrollContext::new(grid_wrapper))
                .insert(
                    ResponsiveLocation::new()
                        .left(stem().left() + 116.px())
                        .width((element_label - 4).px())
                        .height((element_label - 4).px())
                        .bottom(stem().bottom() - 4.px()),
                )
                .insert(EvaluateCore::recursive())
                .id(); // circle-panel color-changing
            let g = tree
                .spawn(Leaf::new().stem(Some(view)).elevation(-1))
                .insert(
                    ResponsiveLocation::new()
                        .left(stem().left())
                        .top(stem().top())
                        .width(canvas_size.0.px())
                        .height(canvas_size.1.px()),
                )
                .id(); // genome
            tree.entity(g).insert(GenomeView {
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
            let game = Game::new(&mut tree, grid_wrapper, g, game_grid, canvas_size);
            tree.entity(g)
                .insert(game)
                .insert(Running(false))
                .insert(Evaluation::default())
                .insert(NetworkInput::default())
                .insert(NetworkOutput::default())
                .insert(reward)
                .insert(Activations::default())
                .insert(EvaluateCore::recursive());
            runner.population.push(g);
            runner.genome_id_gen += 1;
        }
        let expanded_view = tree.spawn(Leaf::new().stem(Some(root)).elevation(-1)).id();
        // TODO elements of expanded-view (game, network, score-label, finished-signal, switch-view)
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
            grid_wrapper,
            grid,
            game_speed,
            game_speed_decrement,
            game_speed_label,
            game_speed_increment,
            expanded_view,
            gen_stop,
            num_running,
        };
        tree.insert_resource(ids);
    }
}
#[derive(Event)]
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
    pub(crate) finished: i32,
    pub(crate) game_grid: GameGrid,
    pub(crate) canvas_size: (i32, i32),
}
#[derive(Event)]
pub(crate) struct GameSpeedChange(pub(crate) i32);
impl GameSpeedChange {
    pub(crate) fn increment(trigger: Trigger<OnClick>, mut tree: Tree) {
        tree.trigger(GameSpeedChange(1));
    }
    pub(crate) fn decrement(trigger: Trigger<OnClick>, mut tree: Tree) {
        tree.trigger(GameSpeedChange(-1));
    }
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut game_speed: ResMut<GameSpeed>,
        ids: Res<RunnerIds>,
        mut tree: Tree,
    ) {
        game_speed.speed = (game_speed.speed + trigger.event().0).clamp(1, 4);
        tree.entity(ids.game_speed_label)
            .insert(TextValue::new(format!("Speed: {}", game_speed.speed)));
    }
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
        ids: Res<RunnerIds>,
        mut text: Query<&mut TextValue>,
        runner: Res<Runner>,
    ) {
        text.get_mut(ids.species_label).unwrap().0 = format!("Species: {}", runner.species.len());
    }
}
pub(crate) struct StopGeneration {}
impl StopGeneration {
    pub(crate) fn obs(trigger: Trigger<OnClick>, mut tree: Tree, mut runner: ResMut<Runner>) {
        runner.run_to = false;
        for genome in runner.population.iter() {
            tree.entity(*genome).insert(Running(false));
        }
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
    pub(crate) fn obs(trigger: Trigger<OnClick>, mut tree: Tree, mut runner: ResMut<Runner>) {
        runner.requested_generation += 1;
        tree.trigger(UpdateGenerationText {});
    }
}
#[derive(Event)]
pub(crate) struct RunToGeneration {}
impl RunToGeneration {
    pub(crate) fn obs(trigger: Trigger<OnClick>, mut runner: ResMut<Runner>, mut tree: Tree) {
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
pub(crate) struct EvaluateWrapper {}
impl EvaluateWrapper {
    pub(crate) fn obs(trigger: Trigger<OnClick>, mut tree: Tree) {
        tree.trigger(Evaluate {});
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
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        mut runner: ResMut<Runner>,
        ids: Res<RunnerIds>,
        views: Query<&GenomeView>,
    ) {
        let genome = trigger.entity();
        let view = views.get(genome).unwrap();
        tree.entity(genome).insert(Evaluation::default());
        let game = Game::new(
            &mut tree,
            ids.grid_wrapper,
            genome,
            runner.game_grid,
            runner.canvas_size,
        );
        tree.entity(genome).insert(game);
        tree.entity(genome).insert(Running(true));
        tree.entity(view.finished_signal).insert(Blue::base());
        tree.entity(view.score).insert(TextValue::new("Score: 0"));
        runner.finished = (runner.finished - 1).max(0);
        tree.entity(ids.num_running).insert(TextValue::new(format!(
            "Running: {}",
            runner.population.len() - runner.finished as usize
        )));
    }
}
#[derive(Event)]
pub(crate) struct ProcessWrapper {}
impl ProcessWrapper {
    pub(crate) fn obs(trigger: Trigger<OnClick>, mut tree: Tree) {
        tree.trigger(Process {});
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
        runner.generation += 1;
        // max-depth
        tree.trigger(MaxDepthCheck {});
        // speciate
        tree.trigger(Speciate {});
        let mut reevaluate = false;
        if runner.run_to {
            if runner.generation < runner.requested_generation {
                reevaluate = true;
            } else {
                runner.requested_generation = runner.generation + 1;
                runner.run_to = false;
            }
        }
        tree.trigger(UpdateGenerationText {});
        tree.trigger(UpdateSpeciesCountText {});
        if reevaluate {
            tree.trigger(Evaluate {});
        }
    }
}
