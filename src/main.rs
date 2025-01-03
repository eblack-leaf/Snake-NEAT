mod intro;
mod overview;
mod runner;
mod section;

use crate::overview::Overview;
use foliage::tree::EcsExtension;
use foliage::twig::Twig;
use foliage::Foliage;
use overview::VIEW_AREA;
fn main() {
    let mut foliage = Foliage::new();
    foliage.set_desktop_size(VIEW_AREA);
    foliage.attach_root::<runner::Runner>();
    foliage.ecs().branch(Twig::new(Overview {}));
    foliage.photosynthesize();
}
