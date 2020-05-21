use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::ops::Mul;

pub struct Damage(pub f32);

impl Component for Damage {
    type Storage = DenseVecStorage<Self>;
}
