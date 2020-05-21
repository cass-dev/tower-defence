use amethyst::core::math::Point2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Path {
    pub points: Vec<Point2<f32>>,
}

impl Path {
    pub fn new(points: Vec<Point2<f32>>) -> Self {
        Path { points }
    }
}

impl Component for Path {
    type Storage = DenseVecStorage<Self>;
}
