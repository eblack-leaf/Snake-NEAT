use crate::runner::NodeId;
use foliage::bevy_ecs;
use foliage::bevy_ecs::component::Component;
#[derive(Component, Copy, Clone)]
pub(crate) struct Node {
    pub(crate) id: NodeId,
    pub(crate) ty: NodeType,
    pub(crate) value: f32,
}

impl Node {
    pub(crate) fn new() -> Self {
        Self {
            id: 0,
            ty: NodeType::Hidden,
            value: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum NodeType {
    Input,
    Output,
    Bias,
    Hidden,
}
