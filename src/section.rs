use crate::intro::{intro_in, intro_out, IntroIds};
use crate::overview::{IdTable, SECTION_OUT_END, SELECTOR_DIM, UNSELECTED_OUTLINE_WEIGHT};
use foliage::anim::Animation;
use foliage::bevy_ecs;
use foliage::bevy_ecs::event::Event;
use foliage::bevy_ecs::prelude::{Component, Res, Trigger};
use foliage::bevy_ecs::system::{Query, ResMut, Resource};
use foliage::interaction::OnClick;
use foliage::opacity::Opacity;
use foliage::panel::OutlineWeight;
use foliage::time::OnEnd;
use foliage::tree::{EcsExtension, Tree};

#[derive(Event)]
pub(crate) struct SectionIn {
    pub(crate) id: usize,
}
impl SectionIn {
    pub(crate) fn obs(trigger: Trigger<Self>, id_table: ResMut<IdTable>, mut tree: Tree) {
        let section_root = id_table
            .section_roots
            .get(trigger.event().id)
            .copied()
            .unwrap();
        match trigger.event().id {
            0 => {
                intro_in(&mut tree, section_root);
            }
            _ => {}
        }
    }
}
#[derive(Event)]
pub(crate) struct SectionOut {
    pub(crate) id: usize,
}
#[derive(Component, Copy, Clone)]
pub(crate) struct OutSection {
    pub(crate) id: usize,
}
impl SectionOut {
    pub(crate) fn obs(trigger: Trigger<Self>, id_table: Res<IdTable>, mut tree: Tree) {
        let section_root = id_table
            .section_roots
            .get(trigger.event().id)
            .copied()
            .unwrap();
        let seq = tree.start_sequence(|seq| {
            let animation = Animation::new(Opacity::new(0.0))
                .start(0)
                .end(SECTION_OUT_END);
            seq.animate(animation.clone().targeting(section_root));
            seq.on_end(Self::end);
        });
        tree.entity(seq).insert(OutSection {
            id: trigger.event().id,
        });
    }
    pub(crate) fn end(
        trigger: Trigger<OnEnd>,
        intro_ids: Res<IntroIds>,
        mut tree: Tree,
        query: Query<&OutSection>,
    ) {
        match query.get(trigger.entity()).copied().unwrap().id {
            0 => {
                intro_out(&mut tree, &intro_ids);
            }
            _ => {}
        }
    }
}
#[derive(Resource, Copy, Clone, Default)]
pub(crate) struct CurrentSection {
    pub(crate) id: usize,
}
#[derive(Event, Copy, Clone)]
pub(crate) struct SelectSection {
    pub(crate) id: usize,
}
impl SelectSection {
    pub(crate) fn obs(
        trigger: Trigger<Self>,
        mut tree: Tree,
        id_table: Res<IdTable>,
        current: Res<CurrentSection>,
    ) {
        let selected = trigger.event().id;
        if current.id != selected {
            tree.trigger(SectionOut { id: current.id });
            tree.start_sequence(|seq| {
                seq.animate(
                    Animation::new(OutlineWeight::new(UNSELECTED_OUTLINE_WEIGHT))
                        .start(0)
                        .end(SECTION_OUT_END)
                        .targeting(id_table.section_buttons.get(current.id).copied().unwrap()),
                );
            });
        }
        tree.start_sequence(|seq| {
            seq.animate(
                Animation::new(OutlineWeight::new(SELECTOR_DIM))
                    .start(0)
                    .end(SECTION_OUT_END)
                    .targeting(id_table.section_buttons.get(selected).copied().unwrap()),
            );
        });
        tree.trigger(SectionIn { id: selected });
        tree.insert_resource(CurrentSection { id: selected });
    }
}
pub(crate) struct SelectObs<const N: usize> {}
impl<const N: usize> SelectObs<N> {
    pub(crate) fn obs(_trigger: Trigger<OnClick>, mut tree: Tree) {
        tree.trigger(SelectSection { id: N });
    }
}
