use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::ops::Mul;
use crate::components::Damage;
use std::cmp::Ordering;

pub struct Health(pub f32);

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}

impl std::ops::SubAssign<&Damage> for &mut Health {
    fn sub_assign(&mut self, rhs: &Damage) {
        self.0 -= rhs.0;
    }
}
