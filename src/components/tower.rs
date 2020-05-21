use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;

use amethyst::{
    assets::Handle,
    core::math::Vector3,
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheet},
};

pub struct Tower {
    pub range: f32,
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}
