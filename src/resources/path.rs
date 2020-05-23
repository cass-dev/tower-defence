use amethyst::core::math::Point2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::ops::Index;

pub struct Path(pub Vec<Point2<f32>>);

impl Default for Path {
    fn default() -> Self {
        Path(vec![])
    }
}

impl Path {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Path {
    type Output = Point2<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
