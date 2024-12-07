use crate::runner::Innovation;
use foliage::bevy_ecs;
use foliage::bevy_ecs::component::Component;
use foliage::bevy_ecs::entity::Entity;
#[derive(Component, Copy, Clone)]
pub(crate) struct Connection {
    pub(crate) weight: f32,
    pub(crate) innovation: Innovation,
    pub(crate) enabled: bool,
    pub(crate) from: Entity,
    pub(crate) to: Entity,
}
