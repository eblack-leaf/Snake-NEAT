use crate::runner::{Innovation, NodeId};
use foliage::bevy_ecs;
use foliage::bevy_ecs::component::Component;
#[derive(Component, Copy, Clone)]
pub(crate) struct Connection {
    pub(crate) weight: f32,
    pub(crate) innovation: Innovation,
    pub(crate) enabled: bool,
    pub(crate) from: NodeId,
    pub(crate) to: NodeId,
}
impl Connection {
    pub(crate) fn new(from: NodeId, to: NodeId, weight: f32, innovation: Innovation) -> Self {
        Self {
            from,
            to,
            weight,
            enabled: true,
            innovation,
        }
    }
}
