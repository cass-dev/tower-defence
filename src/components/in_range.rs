use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::ops::Mul;

pub struct InRange {
    pub other: Entity,
}

impl Component for InRange {
    type Storage = DenseVecStorage<Self>;
}
