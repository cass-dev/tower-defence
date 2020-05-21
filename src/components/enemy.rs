use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;

use amethyst::{
    assets::Handle,
    core::math::Vector3,
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheet},
};

pub struct Enemy {
    pub path_index: Option<usize>,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy { path_index: None }
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
