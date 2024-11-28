use foliage::bevy_ecs;
use foliage::bevy_ecs::prelude::Entity;
use foliage::bevy_ecs::system::Resource;
use foliage::color::{Grey, Monochromatic};
use foliage::leaf::Leaf;
use foliage::text::{FontSize, Text};
use foliage::tree::Tree;
use foliage::twig::{Branch, Twig};
pub(crate) struct Intro {}
#[derive(Resource)]
pub(crate) struct IntroIds {
    pub(crate) title: Entity,
    pub(crate) desc: Entity,
    pub(crate) summary: Entity,
}
impl IntroIds {
    pub(crate) fn new(title: Entity, desc: Entity, summary: Entity) -> Self {
        Self {
            title,
            desc,
            summary,
        }
    }
}
impl Branch for Intro {
    type Handle = ();
    fn grow(_twig: Twig<Self>, tree: &mut Tree) -> Self::Handle {
        let title = tree
            .spawn(Leaf::new())
            .insert(Text::new("NEAT", FontSize::new(48), Grey::plus_two()).centered())
            .id();
        let desc = tree
            .spawn(Leaf::new())
            .insert(
                Text::new(
                    "Neuro-Evolution of Augmented Topologies",
                    FontSize::new(32),
                    Grey::plus_one(),
                )
                .centered(),
            )
            .id();
        let summary = tree
            .spawn(Leaf::new())
            .insert(Text::new("", FontSize::new(14), Grey::plus_one()))
            .id();
        let ids = IntroIds::new(title, desc, summary);
        tree.insert_resource(ids);
    }
}
