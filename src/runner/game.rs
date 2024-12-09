use crate::runner::environment::Environment;
use crate::runner::genome::{Activate, Evaluation, Genome, NetworkInput, NetworkOutput, Reward};
use crate::runner::{GenomeView, Process, Runner, RunnerIds};
use foliage::bevy_ecs;
use foliage::bevy_ecs::component::{ComponentHooks, ComponentId, StorageType};
use foliage::bevy_ecs::event::Event;
use foliage::bevy_ecs::prelude::{Component, Entity, Resource, Trigger};
use foliage::bevy_ecs::query::With;
use foliage::bevy_ecs::system::{Query, Res, ResMut};
use foliage::bevy_ecs::world::DeferredWorld;
use foliage::color::{Color, Grey, Monochromatic, Orange};
use foliage::grid::aspect::stem;
use foliage::grid::responsive::ResponsiveLocation;
use foliage::grid::unit::TokenUnit;
use foliage::grid::Grid;
use foliage::leaf::{EvaluateCore, Leaf};
use foliage::panel::{Panel, Rounding};
use foliage::text::TextValue;
use foliage::time::{Time, TimeDelta};
use foliage::tree::Tree;

#[derive(Resource, Clone)]
pub(crate) struct GameSpeed {
    pub(crate) speed: i32,
    pub(crate) delta: TimeDelta,
}
impl GameSpeed {
    pub(crate) const MIN: i32 = 1;
    pub(crate) const MAX: i32 = 4;
    pub(crate) fn new(speed: i32) -> Self {
        Self {
            speed,
            delta: TimeDelta::default(),
        }
    }
    pub(crate) fn frames_to_skip(&self) -> TimeDelta {
        match self.speed {
            1 => TimeDelta::from_millis(1000 / 24),
            2 => TimeDelta::from_millis(1000 / 36),
            3 => TimeDelta::from_millis(1000 / 48),
            4 => TimeDelta::from_millis(1000 / 60),
            _ => panic!(),
        }
    }
    pub(crate) fn paced_execution(&mut self, time: &Time) -> bool {
        self.delta += time.frame_diff();
        let frames_to_skip = self.frames_to_skip();
        let val = self.delta >= frames_to_skip;
        if val {
            // println!(
            //     "delta {} | {}",
            //     self.delta.as_millis(),
            //     frames_to_skip.as_millis()
            // );
            self.delta = TimeDelta::default();
        }
        val
    }
}
#[derive(Clone)]
pub(crate) struct Game {
    pub(crate) snake: Snake,
    pub(crate) food: Segment,
    pub(crate) canvas: Entity,
    pub(crate) grid: GameGrid,
    pub(crate) collected_food: bool,
    pub(crate) last_tail_location: Location,
    pub(crate) wrapper: Entity,
    pub(crate) can_move_towards_food: bool,
    pub(crate) neighbor_distances: Vec<f32>,
}
#[derive(Copy, Clone)]
pub(crate) struct RewardStatus {
    pub(crate) can_move_towards_food: bool,
    pub(crate) moved_towards_food: bool,
    pub(crate) collected_food: bool,
}
#[derive(Copy, Clone)]
pub(crate) struct GameGrid {
    pub(crate) grid: (i32, i32),
}

impl GameGrid {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Self { grid: (x, y) }
    }
}

impl Game {
    pub(crate) const STARTING_SEGMENTS: i32 = 6;
    pub(crate) fn new(
        tree: &mut Tree,
        wrapper: Entity,
        g: Entity,
        game_grid: GameGrid,
        canvas_size: (i32, i32),
    ) -> Self {
        let canvas = tree
            .spawn(Leaf::new().stem(Some(g)).elevation(-1))
            // .insert(ScrollContext::new(wrapper))
            .insert(Panel::new(Rounding::default(), Color::WHITE))
            .insert(
                ResponsiveLocation::new()
                    .left(stem().left())
                    .top(stem().top())
                    .width(canvas_size.0.px())
                    .height(canvas_size.1.px()),
            )
            .insert(Grid::new(game_grid.grid.0 as u32, game_grid.grid.1 as u32).gap((0, 0)))
            .insert(EvaluateCore::recursive())
            .id();
        let mut snake = Snake {
            segments: vec![],
            direction: Direction::Right,
        };
        let start = Location::new(20, 15);
        for s in 0..Self::STARTING_SEGMENTS {
            let mut location = Location::default();
            location.x = start.x - s;
            location.y = start.y;
            let panel = tree
                .spawn(Leaf::new().stem(Some(canvas)).elevation(-1))
                // .insert(ScrollContext::new(wrapper))
                .insert(Panel::new(Rounding::default(), Grey::minus_two()))
                .insert(
                    ResponsiveLocation::new()
                        .left(location.x.column().begin().of(stem()))
                        .right(location.x.column().end().of(stem()))
                        .top(location.y.row().begin().of(stem()))
                        .bottom(location.y.row().end().of(stem())),
                )
                .insert(EvaluateCore::recursive())
                .id();
            snake.segments.push(Segment { panel, location });
        }
        let location = Location::new(40, 15);
        let food = Segment {
            panel: tree
                .spawn(Leaf::new().stem(Some(canvas)).elevation(-1))
                .insert(Panel::new(Rounding::default(), Orange::base()))
                .insert(
                    ResponsiveLocation::new()
                        .left(location.x.column().begin().of(stem()))
                        .right(location.x.column().end().of(stem()))
                        .top(location.y.row().begin().of(stem()))
                        .bottom(location.y.row().end().of(stem())),
                )
                // .insert(ScrollContext::new(wrapper))
                .insert(EvaluateCore::recursive())
                .id(),
            location: Location::default(),
        };
        let last = snake.segments.last().as_ref().unwrap().location;
        Self {
            snake,
            food,
            canvas,
            grid: game_grid,
            collected_food: false,
            last_tail_location: last,
            wrapper,
            can_move_towards_food: false,
            neighbor_distances: vec![],
        }
    }
    pub(crate) fn distance(a: Location, b: Location) -> f32 {
        ((a.x as f32 - b.x as f32).powi(2) + (a.y as f32 - b.y as f32).powi(2)).sqrt()
    }
    pub(crate) fn reward_status(&self) -> RewardStatus {
        let mut status = RewardStatus {
            can_move_towards_food: true,
            moved_towards_food: false,
            collected_food: false,
        };
        if self.collected_food {
            status.collected_food = true;
        } else {
            let last = self.snake.segments.get(1).unwrap().location;
            let current = self.snake.segments.get(0).unwrap().location;
            let prev = Self::distance(self.food.location, last);
            let now = Self::distance(self.food.location, current);
            if prev >= now {
                status.moved_towards_food = true;
            }
            status.can_move_towards_food = self.can_move_towards_food;
        }
        status
    }
    fn on_remove(mut world: DeferredWorld, this: Entity, _c: ComponentId) {
        let value = world.get::<Game>(this).unwrap().clone();
        // despawn ids
        for s in value.snake.segments.iter() {
            world.commands().entity(s.panel).despawn();
        }
        world.commands().entity(value.food.panel).despawn();
        world.commands().entity(value.canvas).despawn();
    }
}
impl Component for Game {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_remove(Self::on_remove);
    }
}
#[derive(Component, Copy, Clone)]
pub(crate) struct Running(pub(crate) bool);
pub(crate) fn run(
    games: Query<(Entity, &Running), With<Game>>,
    mut speed: ResMut<GameSpeed>,
    time: Res<Time>,
    mut tree: Tree,
) {
    let paced = speed.paced_execution(&time);
    for (entity, running) in games.iter() {
        if running.0 && paced {
            tree.trigger_targets(SetNetworkInput {}, entity);
            tree.trigger_targets(Activate {}, entity);
            tree.trigger_targets(MoveWithNetworkOutput {}, entity);
            tree.trigger_targets(ComputeReward {}, entity);
        }
    }
}
#[derive(Event)]
pub(crate) struct SetNetworkInput {}
impl SetNetworkInput {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        mut inputs: Query<&mut NetworkInput>,
        mut games: Query<&mut Game>,
    ) {
        // evaluate state of game + set NetworkInput
        let mut input = inputs.get_mut(trigger.entity()).unwrap();
        let mut game = games.get_mut(trigger.entity()).unwrap();
        let head = game.snake.segments.get(0).unwrap().location;
        let mut neighbors = vec![head; 4];
        neighbors[0].x -= 1;
        neighbors[1].y += 1;
        neighbors[2].x += 1;
        neighbors[3].y -= 1;
        let neighbor_intersects_tail = neighbors
            .iter()
            .map(|n| {
                game.snake
                    .segments
                    .iter()
                    .find(|s| s.location == *n)
                    .is_some()
            })
            .collect::<Vec<_>>();
        game.neighbor_distances = neighbors
            .iter()
            .map(|n| Game::distance(*n, game.food.location))
            .collect::<Vec<_>>();
        println!("intersect: {:?}", neighbor_intersects_tail);
        println!("neighbors: {:?}", neighbors);
        println!("head: {:?}", head);
        let to_food = Location::new(game.food.location.x - head.x, game.food.location.y - head.y);
        match game.snake.direction {
            Direction::Left => {
                input.is_food_forward = to_food.x.is_negative();
                input.is_food_left = to_food.y.is_positive();
                input.is_food_right = to_food.y.is_negative();
                if !neighbor_intersects_tail[0] && head.x != 0 {
                    input.can_move_forward = true;
                }
                if !neighbor_intersects_tail[1] && head.y + 1 != game.grid.grid.1 {
                    input.can_move_left = true;
                }
                if !neighbor_intersects_tail[3] && head.y != 0 {
                    input.can_move_right = true;
                }
            }
            Direction::Right => {
                input.is_food_forward = to_food.x.is_positive();
                input.is_food_left = to_food.y.is_negative();
                input.is_food_right = to_food.y.is_positive();
                if !neighbor_intersects_tail[2] && head.x + 1 != game.grid.grid.0 {
                    input.can_move_forward = true;
                }
                if !neighbor_intersects_tail[1] && head.y + 1 != game.grid.grid.1 {
                    input.can_move_right = true;
                }
                if !neighbor_intersects_tail[3] && head.y != 0 {
                    input.can_move_left = true;
                }
            }
            Direction::Up => {
                input.is_food_forward = to_food.y.is_negative();
                input.is_food_left = to_food.x.is_negative();
                input.is_food_right = to_food.x.is_positive();
                if !neighbor_intersects_tail[3] && head.y != 0 {
                    input.can_move_forward = true;
                }
                if !neighbor_intersects_tail[0] && head.x != 0 {
                    input.can_move_left = true;
                }
                if !neighbor_intersects_tail[2] && head.x + 1 != game.grid.grid.0 {
                    input.can_move_right = true;
                }
            }
            Direction::Down => {
                input.is_food_forward = to_food.y.is_positive();
                input.is_food_left = to_food.x.is_positive();
                input.is_food_right = to_food.x.is_negative();
                if !neighbor_intersects_tail[1] && head.y + 1 != game.grid.grid.1 {
                    input.can_move_forward = true;
                }
                if !neighbor_intersects_tail[2] && head.x + 1 != game.grid.grid.0 {
                    input.can_move_right = true;
                }
                if !neighbor_intersects_tail[0] && head.x != 0 {
                    input.can_move_left = true;
                }
            }
        }
    }
}
#[derive(Event)]
pub(crate) struct MoveWithNetworkOutput {}
impl MoveWithNetworkOutput {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        inputs: Query<&NetworkInput>,
        outputs: Query<&NetworkOutput>,
        mut games: Query<&mut Game>,
        mut runner: ResMut<Runner>,
        views: Query<&GenomeView>,
        ids: Res<RunnerIds>,
    ) {
        let mut game = games.get_mut(trigger.entity()).unwrap();
        let view = views.get(trigger.entity()).unwrap();
        let mut new_head = game.snake.segments.get(0).unwrap().location;
        let current_head = new_head;
        let input = inputs.get(trigger.entity()).unwrap();
        let mut neighbors = vec![new_head; 4];
        neighbors[0].x -= 1;
        neighbors[1].y += 1;
        neighbors[2].x += 1;
        neighbors[3].y -= 1;
        // move snake
        game.can_move_towards_food = false;
        let output = outputs.get(trigger.entity()).unwrap();
        if output.move_left {
            // left
            match game.snake.direction {
                Direction::Left => {
                    new_head = neighbors[1];
                    game.snake.direction = Direction::Down;
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(0).unwrap();
                    if projected < current && input.can_move_forward {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(3).unwrap();
                    if projected < current && input.can_move_right {
                        game.can_move_towards_food = true;
                    }
                }
                Direction::Right => {
                    new_head = neighbors[3];
                    game.snake.direction = Direction::Up;
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(1).unwrap();
                    if projected < current && input.can_move_right {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(2).unwrap();
                    if projected < current && input.can_move_forward {
                        game.can_move_towards_food = true;
                    }
                }
                Direction::Up => {
                    new_head = neighbors[0];
                    game.snake.direction = Direction::Left;
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(2).unwrap();
                    if projected < current && input.can_move_right {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(3).unwrap();
                    if projected < current && input.can_move_forward {
                        game.can_move_towards_food = true;
                    }
                }
                Direction::Down => {
                    new_head = neighbors[2];
                    game.snake.direction = Direction::Right;
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(1).unwrap();
                    if projected < current && input.can_move_forward {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(0).unwrap();
                    if projected < current && input.can_move_right {
                        game.can_move_towards_food = true;
                    }
                }
            }
        } else if output.move_right {
            // right
            match game.snake.direction {
                Direction::Left => {
                    new_head = neighbors[3];
                    game.snake.direction = Direction::Up;
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(0).unwrap();
                    if projected < current && input.can_move_forward {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(1).unwrap();
                    if projected < current && input.can_move_left {
                        game.can_move_towards_food = true;
                    }
                }
                Direction::Right => {
                    new_head = neighbors[1];
                    game.snake.direction = Direction::Down;
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(2).unwrap();
                    if projected < current && input.can_move_forward {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(3).unwrap();
                    if projected < current && input.can_move_left {
                        game.can_move_towards_food = true;
                    }
                }
                Direction::Up => {
                    new_head = neighbors[2];
                    game.snake.direction = Direction::Right;
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(3).unwrap();
                    if projected < current && input.can_move_forward {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(0).unwrap();
                    if projected < current && input.can_move_left {
                        game.can_move_towards_food = true;
                    }
                }
                Direction::Down => {
                    new_head = neighbors[0];
                    game.snake.direction = Direction::Left;
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(1).unwrap();
                    if projected < current && input.can_move_forward {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(0).unwrap();
                    if projected < current && input.can_move_left {
                        game.can_move_towards_food = true;
                    }
                }
            }
        } else {
            // forward
            match game.snake.direction {
                Direction::Left => {
                    new_head = neighbors[0];
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(1).unwrap();
                    if projected < current && input.can_move_left {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(3).unwrap();
                    if projected < current && input.can_move_right {
                        game.can_move_towards_food = true;
                    }
                }
                Direction::Right => {
                    new_head = neighbors[2];
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(3).unwrap();
                    if projected < current && input.can_move_left {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(1).unwrap();
                    if projected < current && input.can_move_right {
                        game.can_move_towards_food = true;
                    }
                }
                Direction::Up => {
                    new_head = neighbors[3];
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(0).unwrap();
                    if projected < current && input.can_move_left {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(2).unwrap();
                    if projected < current && input.can_move_right {
                        game.can_move_towards_food = true;
                    }
                }
                Direction::Down => {
                    new_head = neighbors[1];
                    let current = Game::distance(current_head, game.food.location);
                    let projected = *game.neighbor_distances.get(2).unwrap();
                    if projected < current && input.can_move_left {
                        game.can_move_towards_food = true;
                    }
                    let projected = *game.neighbor_distances.get(0).unwrap();
                    if projected < current && input.can_move_right {
                        game.can_move_towards_food = true;
                    }
                }
            }
        }
        if game
            .snake
            .segments
            .iter()
            .find(|s| s.location == new_head)
            .is_some()
            || new_head.x < 0
            || new_head.x >= game.grid.grid.0
            || new_head.y < 0
            || new_head.y >= game.grid.grid.1
        {
            tree.entity(trigger.entity()).insert(Running(false));
            tree.entity(view.finished_signal).insert(Orange::base());
            runner.finished += 1;
            tree.entity(ids.num_running).insert(TextValue::new(format!(
                "Running: {}",
                runner.population.len() - runner.finished as usize
            )));
        }
        let mut new_segment_locations = vec![new_head];
        for seg in game.snake.segments.iter_mut() {
            new_segment_locations.push(seg.location);
        }
        if new_head == game.food.location {
            game.collected_food = true;
            let segment = Segment {
                panel: tree
                    .spawn(Leaf::new().stem(Some(game.canvas)).elevation(-1))
                    // .insert(ScrollContext::new(game.wrapper))
                    .insert(Panel::new(Rounding::default(), Grey::minus_two()))
                    .id(),
                location: game.last_tail_location,
            };
            game.snake.segments.push(segment);
            game.food.location = Location::new(0, 0);
            tree.entity(game.food.panel)
                .insert(
                    ResponsiveLocation::new()
                        .left(game.food.location.x.column().begin().of(stem()))
                        .right(game.food.location.x.column().end().of(stem()))
                        .top(game.food.location.y.row().begin().of(stem()))
                        .bottom(game.food.location.y.row().end().of(stem())),
                )
                .insert(EvaluateCore::recursive());
        } else {
            let _ = new_segment_locations.pop();
        }
        for (i, seg) in new_segment_locations.iter().enumerate() {
            game.snake.segments.get_mut(i).unwrap().location = *seg;
            tree.entity(game.snake.segments.get(i).unwrap().panel)
                .insert(
                    ResponsiveLocation::new()
                        .left(seg.x.column().begin().of(stem()))
                        .right(seg.x.column().end().of(stem()))
                        .top(seg.y.row().begin().of(stem()))
                        .bottom(seg.y.row().end().of(stem())),
                )
                .insert(EvaluateCore::recursive());
        }
    }
}
#[derive(Event)]
pub(crate) struct ComputeReward {}
impl ComputeReward {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        games: Query<&Game>,
        mut runner: ResMut<Runner>,
        environment: Res<Environment>,
        mut rewards: Query<&mut Reward>,
        mut evaluations: Query<(Entity, &mut Evaluation)>,
        genomes: Query<&Genome>,
        views: Query<&GenomeView>,
        mut texts: Query<&mut TextValue>,
    ) {
        let (_, mut eval) = evaluations.get_mut(trigger.entity()).unwrap();
        let mut reward = rewards.get_mut(trigger.entity()).unwrap();
        let status = games.get(trigger.entity()).unwrap().reward_status();
        reward.can_move_towards_food = status.can_move_towards_food;
        reward.moved_towards_food = status.moved_towards_food;
        reward.collected_food = status.collected_food;
        eval.num_turns_taken += 1;
        if eval.num_turns_taken >= environment.max_turns {
            tree.entity(trigger.entity()).insert(Running(false));
        }
        eval.fitness += reward.value();
        let view = views.get(trigger.entity()).unwrap();
        texts.get_mut(view.score).unwrap().0 = format!("Score: {}", eval.fitness);
        drop(eval);
        if runner.finished == environment.population_count {
            // give info to best
            let current_best = evaluations
                .iter()
                .map(|a| (a.0, *a.1))
                .max_by(|a, b| a.1.fitness.partial_cmp(&b.1.fitness).unwrap())
                .unwrap();
            if current_best.1.fitness > runner.best.as_ref().unwrap().1.fitness {
                runner
                    .best
                    .replace((genomes.get(current_best.0).unwrap().clone(), current_best.1));
                // TODO update best score label + genome
            }
            if runner.run_to {
                tree.trigger(Process {});
            }
        }
    }
}
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub(crate) struct Location {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Segment {
    pub(crate) panel: Entity,
    pub(crate) location: Location,
}
#[derive(Component, Clone)]
pub(crate) struct Snake {
    pub(crate) segments: Vec<Segment>,
    pub(crate) direction: Direction,
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub(crate) enum Direction {
    Left,
    Right,
    Up,
    Down,
}
