use crate::runner::genome::{Genome, NetworkOutput};
use foliage::bevy_ecs;
use foliage::bevy_ecs::prelude::{Component, Entity, Resource};
use foliage::bevy_ecs::system::Query;
use foliage::grid::responsive::{ResponsiveLocation, ResponsiveSection};

#[derive(Resource, Copy, Clone)]
pub(crate) struct GameSpeed(pub(crate) i32);
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
pub(crate) fn run(games: Query<(&Game, &Running, &Genome, &NetworkOutput)>, snakes: Query<&Snake>) {
    for (game, running, genome, output) in games.iter() {
        if running.0 {
            // process w/ output as movement
            // if hit => end [Running(false)]
            // update snake segment locations (animate to next over time-delta)
            // if got food => respawn food
        }
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
