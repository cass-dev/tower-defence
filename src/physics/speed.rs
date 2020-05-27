use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::ops::Mul;

#[derive(Debug)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Speed(0.0)
    }
}

impl Component for Speed {
    type Storage = DenseVecStorage<Self>;
}

impl std::ops::Mul<&Speed> for f32 {
    type Output = f32;

    fn mul(self, rhs: &Speed) -> Self::Output {
        self * rhs.0
    }
}
