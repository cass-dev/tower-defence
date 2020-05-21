use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::ops::Mul;

pub struct Missile {
    pub enemy: Entity,
}

impl Component for Missile {
    type Storage = DenseVecStorage<Self>;
}
