use crate::runner::connection::Connection;
use crate::runner::node::Node;
use crate::runner::{Activation, Depth, Fitness, GenomeId, SpeciesId};
use foliage::bevy_ecs;
use foliage::bevy_ecs::component::Component;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::prelude::Query;
use foliage::tree::Tree;
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
    pub(crate) fn new(tree: &mut Tree, this: Entity, input_size: i32, output_size: i32) -> Self {
        // setup nodes + connections [dependent on genome]
        todo!()
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
#[derive(Component, Default)]
pub(crate) struct NetworkOutput {
    pub(crate) move_left: bool,
    pub(crate) move_right: bool,
}
pub(crate) fn evaluate(
    evaluations: Query<&mut Evaluation>,
    nodes: Query<&Node>,
    connections: Query<&Connection>,
    input: Query<&mut NetworkInput>,
    output: Query<&mut NetworkOutput>,
) {
}
