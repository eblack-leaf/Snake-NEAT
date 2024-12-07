pub(crate) struct CompatibilityFactors {
    pub(crate) c1: f32,
    pub(crate) c2: f32,
    pub(crate) c3: f32,
}

pub(crate) struct Compatibility {
    pub(crate) excess: f32,
    pub(crate) disjoint: f32,
    pub(crate) weight_difference: f32,
    pub(crate) n: f32,
}

impl Compatibility {
    pub(crate) fn new() -> Self {
        Self {
            excess: 0.0,
            disjoint: 0.0,
            weight_difference: 0.0,
            n: 0.0,
        }
    }
    pub(crate) fn distance(&self, factors: &CompatibilityFactors) -> f32 {
        factors.c1 * self.excess / self.n
            + factors.c2 * self.disjoint / self.n
            + factors.c3 * self.weight_difference
    }
}
