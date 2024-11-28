use crate::section::{CurrentSection, SectionIn, SelectObs, SelectSection};
use foliage::bevy_ecs::entity::Entity;
use foliage::bevy_ecs::system::Resource;
use foliage::color::{Grey, Monochromatic};
use foliage::grid::responsive::evaluate::{ScrollContext, Scrollable};
use foliage::grid::responsive::ResponsiveLocation;
use foliage::icon::{Icon, IconRequest};
use foliage::interaction::ClickInteractionListener;
use foliage::leaf::{EvaluateCore, Leaf};
use foliage::panel::{Panel, Rounding};
use foliage::text::{FontSize, Text};
use foliage::tree::Tree;
use foliage::twig::{Branch, Twig};
use foliage::{bevy_ecs, icon_handle};

#[derive(Resource)]
pub(crate) struct IdTable {
    pub(crate) section_buttons: Vec<Entity>,
    pub(crate) section_icons: Vec<Entity>,
    pub(crate) section_lines: Vec<Entity>,
    pub(crate) section_titles: Vec<Entity>,
    pub(crate) section_roots: Vec<Entity>,
    pub(crate) view_root: Entity,
    pub(crate) side_panel_root: Entity,
}
impl IdTable {
    pub(crate) fn new(view_root: Entity, side_panel_root: Entity) -> Self {
        Self {
            section_buttons: vec![],
            section_icons: vec![],
            section_lines: vec![],
            section_titles: vec![],
            section_roots: vec![],
            view_root,
            side_panel_root,
        }
    }
}
pub(crate) struct Overview {}
#[icon_handle]
pub(crate) enum IconHandles {
    Check,
}
pub(crate) const NUM_SECTIONS: usize = 8;
pub(crate) const SELECTOR_DIM: u32 = 32;
pub(crate) const SECTION_OUT_END: u64 = 300;
pub(crate) const UNSELECTED_OUTLINE_WEIGHT: u32 = 3;
pub(crate) const SECTION_TITLES: [&'static str; NUM_SECTIONS] = [
    "Intro",
    "Network",
    "Mutation",
    "Crossover",
    "Activation",
    "Snake",
    "Population",
    "Runner",
];
impl Branch for Overview {
    type Handle = ();

    fn grow(_twig: Twig<Self>, tree: &mut Tree) -> Self::Handle {
        tree.spawn(IconRequest::new(
            IconHandles::Check,
            include_bytes!("assets/check.icon").to_vec(),
        ));
        let view_location = ResponsiveLocation::new();
        let view_root = tree
            .spawn(Leaf::new().elevation(10))
            .insert(view_location)
            .insert(Scrollable::new())
            .id();
        let side_panel_location = ResponsiveLocation::new();
        let side_panel_root = tree
            .spawn(Leaf::new().elevation(10))
            .insert(side_panel_location)
            .insert(Scrollable::new())
            .id();
        let mut id_table = IdTable::new(view_root, side_panel_root);
        for i in 0..NUM_SECTIONS {
            let section_location = ResponsiveLocation::new();
            let section_root = tree
                .spawn(Leaf::new().elevation(0).opacity(0.0))
                .insert(Panel::new(Rounding::all(0.2), Grey::minus_three()))
                .insert(section_location)
                .insert(EvaluateCore::recursive())
                .id();
            id_table.section_roots.push(section_root);
            let panel =
                Panel::new(Rounding::all(1.0), Grey::plus_two()).outline(UNSELECTED_OUTLINE_WEIGHT);
            let location = ResponsiveLocation::new();
            let panel = tree
                .spawn(Leaf::new().elevation(0))
                .insert(panel)
                .insert(location)
                .insert(ScrollContext::new(side_panel_root))
                .insert(ClickInteractionListener::new().as_circle())
                .insert(EvaluateCore::recursive())
                .id();
            match i {
                0 => {
                    tree.entity(panel).observe(SelectObs::<0>::obs);
                }
                1 => {
                    tree.entity(panel).observe(SelectObs::<1>::obs);
                }
                2 => {
                    tree.entity(panel).observe(SelectObs::<2>::obs);
                }
                3 => {
                    tree.entity(panel).observe(SelectObs::<3>::obs);
                }
                4 => {
                    tree.entity(panel).observe(SelectObs::<4>::obs);
                }
                5 => {
                    tree.entity(panel).observe(SelectObs::<5>::obs);
                }
                6 => {
                    tree.entity(panel).observe(SelectObs::<6>::obs);
                }
                7 => {
                    tree.entity(panel).observe(SelectObs::<7>::obs);
                }
                8 => {
                    tree.entity(panel).observe(SelectObs::<8>::obs);
                }
                _ => {}
            };
            id_table.section_buttons.push(panel);
            let icon = Icon::new(IconHandles::Check, Grey::base());
            let icon_location = ResponsiveLocation::new();
            let icon = tree
                .spawn(Leaf::new().elevation(-1))
                .insert(icon_location)
                .insert(icon)
                .insert(EvaluateCore::recursive())
                .id();
            id_table.section_icons.push(icon);
            let text = Text::new(SECTION_TITLES[i], FontSize::new(14), Grey::plus_two());
            let text_location = ResponsiveLocation::new();
            let text = tree
                .spawn(Leaf::new().elevation(0))
                .insert(text)
                .insert(text_location)
                .insert(EvaluateCore::recursive())
                .id();
            id_table.section_titles.push(text);
        }
        tree.insert_resource(id_table);
        tree.insert_resource(CurrentSection::default());
        tree.trigger(SelectSection { id: 0 });
    }
}
