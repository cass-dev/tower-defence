use amethyst::core::math::{Unit, Vector2};
use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Copy, Clone)]
pub struct Velocity(pub Option<Unit<Vector2<f32>>>);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(None)
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
