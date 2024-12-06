mod intro;
mod overview;
mod runner;
mod section;

use crate::overview::Overview;
use crate::runner::Environment;
use foliage::tree::EcsExtension;
use foliage::twig::Twig;
use foliage::Foliage;

const VIEW_AREA: (f32, f32) = (1600.0, 800.0);
fn main() {
    let mut foliage = Foliage::new();
    foliage.set_desktop_size(VIEW_AREA);
    let mut environment = Environment::new();
    environment.population_count = 150;
    environment.input_size = 6;
    environment.output_size = 2;
    foliage.ecs().insert_resource(environment);
    foliage.ecs().branch(Twig::new(Overview {}));
    foliage.photosynthesize();
}
