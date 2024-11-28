mod intro;
mod overview;
mod section;

use crate::overview::Overview;
use foliage::tree::EcsExtension;
use foliage::twig::Twig;
use foliage::Foliage;

const VIEW_AREA: (f32, f32) = (1600.0, 800.0);
fn main() {
    let mut foliage = Foliage::new();
    foliage.set_desktop_size(VIEW_AREA);
    foliage.ecs().branch(Twig::new(Overview {}));
    foliage.photosynthesize();
}
