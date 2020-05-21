use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use std::ops::Mul;

pub struct FireRate {
    fire_rate: f32,
    last_fire_time: f32,
}

impl FireRate {
    pub fn new(fire_rate: f32) -> Self {
        FireRate {
            fire_rate,
            last_fire_time: 0.0,
        }
    }

    pub fn can_fire(&mut self, time_delta: f32) -> bool {
        self.last_fire_time -= time_delta;
        if self.last_fire_time <= 0.0 {
            self.last_fire_time = self.fire_rate;
            true
        } else {
            false
        }
    }
}

impl Component for FireRate {
    type Storage = DenseVecStorage<Self>;
}
