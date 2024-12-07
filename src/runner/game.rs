use crate::runner::genome::{Activate, Genome, NetworkOutput};
use foliage::bevy_ecs;
use foliage::bevy_ecs::event::Event;
use foliage::bevy_ecs::prelude::{Component, Entity, Resource, Trigger};
use foliage::bevy_ecs::query::With;
use foliage::bevy_ecs::system::{Query, Res, ResMut};
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
#[derive(Component, Copy, Clone)]
pub(crate) struct Game {
    pub(crate) snake: Entity,
    pub(crate) food: Entity,
    pub(crate) canvas: Entity,
    pub(crate) grid: (i32, i32),
    pub(crate) updated: bool,
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
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree) {
        // evaluate state of game + set NetworkInput
    }
}
#[derive(Event)]
pub(crate) struct MoveWithNetworkOutput {}
impl MoveWithNetworkOutput {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree) {
        // move snake
        // if hit => end [Running(false)] + runner.finished += 1
        // update snake segment locations (animate to next over frames_to_skip - 0.01)
        // if got food => respawn food
    }
}
#[derive(Event)]
pub(crate) struct ComputeReward {}
impl ComputeReward {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree) {
        // get evaluation + reward comps
        // use to calc fitness of turn
        // if runner.finished == runner.population.len() && runner.run_to && runner.gen < runner.requested_gen => trigger(Process{});
    }
}
#[derive(Copy, Clone)]
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
    pub(crate) location: Location,
    pub(crate) segments: Vec<Segment>,
}
