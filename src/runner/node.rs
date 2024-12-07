use crate::runner::NodeId;
use foliage::bevy_ecs;
use foliage::bevy_ecs::component::Component;
#[derive(Component, Copy, Clone)]
pub(crate) struct Node {
    pub(crate) id: NodeId,
    pub(crate) ty: NodeType,
}

impl Node {
    pub(crate) fn new() -> Self {
        Self {
            id: 0,
            ty: NodeType::Hidden,
        }
    }
    pub(crate) fn explicit(id: NodeId, ty: NodeType) -> Self {
        Self { id, ty }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) enum NodeType {
    Input,
    Output,
    Bias,
    Hidden,
}
