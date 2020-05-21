use amethyst::core::{math::Vector2, transform::Transform};
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::ops::Mul;

pub struct InRange {
    pub other: Entity,
    pub other_transform: Transform,
}

impl Component for InRange {
    type Storage = DenseVecStorage<Self>;
}
