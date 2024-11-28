use crate::overview::SECTION_OUT_END;
use foliage::anim::Animation;
use foliage::bevy_ecs;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::prelude::Resource;
use foliage::color::{Grey, Monochromatic};
use foliage::grid::responsive::evaluate::ScrollContext;
use foliage::leaf::Leaf;
use foliage::opacity::Opacity;
use foliage::text::{FontSize, Text};
use foliage::tree::{EcsExtension, Tree};
pub(crate) fn intro_in(tree: &mut Tree, section_root: Entity) {
    let title = tree
        .spawn(Leaf::new().stem(Some(section_root)).opacity(0.0))
        .insert(Text::new("NEAT", FontSize::new(48), Grey::plus_two()).centered())
        .insert(ScrollContext::new(section_root))
        .id();
    let desc = tree
        .spawn(Leaf::new().stem(Some(section_root)).opacity(0.0))
        .insert(
            Text::new(
                "Neuro-Evolution of Augmented Topologies",
                FontSize::new(32),
                Grey::plus_one(),
            )
            .centered(),
        )
        .insert(ScrollContext::new(section_root))
        .id();
    let summary = tree
        .spawn(Leaf::new().stem(Some(section_root)).opacity(0.0))
        .insert(Text::new(
            "Summary of NEAT Procedure...",
            FontSize::new(14),
            Grey::plus_one(),
        ))
        .insert(ScrollContext::new(section_root))
        .id();
    let ids = IntroIds::new(title, desc, summary);
    tree.start_sequence(|seq| {
        let animation = Animation::new(Opacity::new(1.0))
            .start(SECTION_OUT_END)
            .end(SECTION_OUT_END + 300);
        seq.animate(animation.clone().targeting(section_root));
        seq.animate(animation.clone().targeting(title));
        seq.animate(
            animation
                .clone()
                .targeting(desc)
                .start(SECTION_OUT_END + 100)
                .end(SECTION_OUT_END + 400),
        );
        seq.animate(
            animation
                .clone()
                .targeting(summary)
                .start(SECTION_OUT_END + 200)
                .end(SECTION_OUT_END + 500),
        );
    });
    tree.insert_resource(ids);
}
pub(crate) fn intro_out(tree: &mut Tree, intro_ids: &IntroIds) {
    tree.entity(intro_ids.title).despawn();
    tree.entity(intro_ids.desc).despawn();
    tree.entity(intro_ids.summary).despawn();
    tree.remove_resource::<IntroIds>();
}
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
