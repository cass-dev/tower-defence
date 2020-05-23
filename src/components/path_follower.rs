use amethyst::core::math::Point2;
use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::ops::Mul;

pub enum PathingState {
    MoveToPoint(usize, Point2<f32>),
    SnapToPoint(usize),
}

pub struct PathFollower {
    pub pathing_state: PathingState,
}

impl Component for PathFollower {
    type Storage = DenseVecStorage<Self>;
}
