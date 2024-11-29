use crate::overview::{IdTable, SECTION_OUT_END};
use foliage::anim::Animation;
use foliage::bevy_ecs;
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::event::Event;
use foliage::bevy_ecs::prelude::{Res, Resource, Trigger};
use foliage::color::{Grey, Monochromatic};
use foliage::grid::aspect::stem;
use foliage::grid::responsive::evaluate::ScrollContext;
use foliage::grid::responsive::ResponsiveLocation;
use foliage::grid::unit::TokenUnit;
use foliage::leaf::{EvaluateCore, Leaf};
use foliage::opacity::Opacity;
use foliage::text::{FontSize, Text};
use foliage::tree::{EcsExtension, Tree};
pub(crate) const SUMMARY_TEXT: &'static str = "This was first developed by Miikkulainen and Stanley in 2002 at the University of Texas at Austin. \
This method mutates the weights and topology of a network starting from the most minimal version \
searching the solution space through mutation and crossover of genomes until the best \
solution is found. The method starts by creating a population of the simplest network (one connection \
from each input to each output and the bias connections) upon which mutations will gradually increase \
the complexity of the networks. The fitness of the network is evaluated every generation to determine \
the fittest organisms. They are grouped into alike categories called species where each member \
competes with all the other members in that species. Each species takes the top 20% of their members, \
evaluated by fitness, and produces the next generation by means of mutation (perturbing weights and \
adding connections) and crossover (combining two members into a new one with traits from each \
parent). Species that do not perform well cannot survive for long and are removed from the possible \
solutions by removing the species’ members. This results in the most optimal solutions at the end of \
many generations.";
#[derive(Event)]
pub(crate) struct IntroIn {
    pub(crate) root: Entity,
}
impl IntroIn {
    pub(crate) fn obs(trigger: Trigger<Self>, mut tree: Tree) {
        let section_root = trigger.event().root;
        let title_location = ResponsiveLocation::new()
            .center_x(stem().center_x())
            .width(50.percent().width().of(stem()))
            .center_y(stem().top() + 48.px())
            .auto_height();
        let title = tree
            .spawn(Leaf::new().stem(Some(section_root)).opacity(0.0))
            .insert(Text::new("NEAT", FontSize::new(48), Grey::plus_three()).centered())
            .insert(ScrollContext::new(section_root))
            .insert(title_location)
            .insert(EvaluateCore::recursive())
            .id();
        let desc_location = ResponsiveLocation::new()
            .top(stem().bottom() + 32.px())
            .center_x(stem().center_x())
            .auto_height()
            .width(500.px());
        let desc = tree
            .spawn(Leaf::new().stem(Some(title)).opacity(0.0))
            .insert(
                Text::new(
                    "Neuro-Evolution of Augmented Topologies",
                    FontSize::new(32),
                    Grey::plus_two(),
                )
                    .centered(),
            )
            .insert(ScrollContext::new(section_root))
            .insert(desc_location)
            .insert(EvaluateCore::recursive())
            .id();
        let summary_location = ResponsiveLocation::new()
            .center_x(stem().center_x())
            .width(90.percent().width().of(stem()))
            .top(stem().bottom() + 16.px())
            .auto_height();
        let summary = tree
            .spawn(Leaf::new().stem(Some(desc)).opacity(0.0))
            .insert(Text::new(SUMMARY_TEXT, FontSize::new(14), Grey::plus_two()))
            .insert(ScrollContext::new(section_root))
            .insert(summary_location)
            .insert(EvaluateCore::recursive())
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
}
#[derive(Event)]
pub(crate) struct IntroOut {

}
impl IntroOut {
    pub(crate) fn obs(_trigger: Trigger<Self>, mut tree: Tree, intro_ids: Res<IntroIds>) {
        tree.entity(intro_ids.title).despawn();
        tree.entity(intro_ids.desc).despawn();
        tree.entity(intro_ids.summary).despawn();
        tree.remove_resource::<IntroIds>();
    }
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
