use crate::runner::environment::Environment;
use crate::runner::genome::{Activate, Evaluation, NetworkInput, NetworkOutput, Reward};
use crate::runner::{Process, Runner};
use foliage::bevy_ecs;
use foliage::bevy_ecs::component::{ComponentHooks, ComponentId, StorageType};
use foliage::bevy_ecs::event::Event;
use foliage::bevy_ecs::prelude::{Component, Entity, Resource, Trigger};
use foliage::bevy_ecs::query::With;
use foliage::bevy_ecs::system::{Query, Res, ResMut};
use foliage::bevy_ecs::world::DeferredWorld;
use foliage::leaf::Leaf;
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
        self.delta -= frames_to_skip;
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
    pub(crate) last_head_location: Location,
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
    pub(crate) fn new(tree: &mut Tree, g: Entity, game_grid: GameGrid) -> Self {
        let mut snake = Snake { segments: vec![] };
        for s in 0..Self::STARTING_SEGMENTS {
            let panel = tree.spawn(Leaf::new().stem(Some(g))).id();
            let location = Location::default();
            snake.segments.push(Segment { panel, location });
        }
        let food = Segment {
            panel: tree.spawn(Leaf::new().stem(Some(g))).id(),
            location: Location::default(),
        };
        let canvas = tree.spawn(Leaf::new().stem(Some(g))).id();
        Self {
            snake,
            food,
            canvas,
            grid: game_grid,
            collected_food: false,
            last_head_location: Default::default(), // same as above for starting head location
        }
    }
    pub(crate) fn reward_status(&self) -> RewardStatus {
        todo!()
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
        game: Query<&Game>,
    ) {
        // evaluate state of game + set NetworkInput
    }
}
#[derive(Event)]
pub(crate) struct MoveWithNetworkOutput {}
impl MoveWithNetworkOutput {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        outputs: Query<&NetworkOutput>,
        mut game: Query<&mut Game>,
    ) {
        // move snake
        // if hit => end [Running(false)] + runner.finished += 1
        // update snake segment locations (animate to next over frames_to_skip - 0.01)
        // if got food => respawn food
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
        mut evaluations: Query<&mut Evaluation>,
    ) {
        let mut eval = evaluations.get_mut(trigger.entity()).unwrap();
        let mut reward = rewards.get_mut(trigger.entity()).unwrap();
        // set reward statuses
        let status = games.get(trigger.entity()).unwrap().reward_status();
        reward.can_move_towards_food = status.can_move_towards_food;
        reward.moved_towards_food = status.moved_towards_food;
        reward.collected_food = status.collected_food;
        eval.num_turns_taken += 1;
        if eval.num_turns_taken >= environment.max_turns {
            tree.entity(trigger.entity()).insert(Running(false));
        }
        eval.fitness += reward.value();
        if runner.finished == environment.population_count && runner.run_to {
            tree.trigger(Process {});
        }
    }
}
#[derive(Copy, Clone, Default)]
pub(crate) struct Location {
    pub(crate) x: i32,
    pub(crate) y: i32,
}
#[derive(Copy, Clone)]
pub(crate) struct Segment {
    pub(crate) panel: Entity,
    pub(crate) location: Location,
}
#[derive(Component, Clone)]
pub(crate) struct Snake {
    pub(crate) segments: Vec<Segment>,
}
