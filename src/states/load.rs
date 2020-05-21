use crate::components::{Enemy, FireRate, Missile, Path, Speed, Tower, Velocity, CircleBounds, Health, Damage};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::systems::{MissileTargetter, TowerFirer};
use crate::{camera, texture, states};
use crate::{components, systems};
use amethyst::ecs::{Dispatcher, DispatcherBuilder};
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::{
    assets::Handle,
    core::math::Vector3,
    renderer::{SpriteRender, SpriteSheet},
};
use amethyst::{
    core::math::{Orthographic3, Point2},
    prelude::*,
    renderer::camera::Projection,
    window::ScreenDimensions,
    assets::{ProgressCounter, Completion},
};
use amethyst::{core::transform::Transform, renderer::Camera, GameData, SimpleState, StateData};

#[derive(Default)]
pub struct Load<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress: ProgressCounter
}

impl<'a, 'b> SimpleState for Load<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        camera::init(data.world);
        components::init(data.world);
        self.progress = texture::init(data.world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Complete => {
                println!("{:?}", self.progress);
                Trans::Switch(Box::new(states::Game::default()))
            },
            Completion::Failed => {
                println!("something bad happened");
                Trans::Quit
            },
            Completion::Loading => Trans::None
        }
    }
}
