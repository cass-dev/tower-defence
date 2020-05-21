use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;

use amethyst::{
    assets::Handle,
    core::math::Vector3,
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheet},
};

pub struct CircleBounds {
    pub(crate) radius: f32,
}

impl Component for CircleBounds {
    type Storage = DenseVecStorage<Self>;
}
