use amethyst::core::math::Point2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Path(pub Vec<Point2<f32>>);

impl Default for Path {
    fn default() -> Self {
        Path(vec![])
    }
}
