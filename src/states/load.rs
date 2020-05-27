use crate::components::{CircleBounds, Damage, Enemy, FireRate, Health, Missile, Tower};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::level::Level;
use crate::systems::{MissileTargetter, TowerFirer};
use crate::{camera, level, states, texture};
use crate::{components, systems};
use amethyst::ecs::{Dispatcher, DispatcherBuilder};
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::{
    assets::AssetStorage,
    assets::{Completion, ProgressCounter},
    core::math::{Orthographic3, Point2},
    prelude::*,
    renderer::camera::Projection,
    window::ScreenDimensions,
};
use amethyst::{
    assets::Handle,
    core::math::Vector3,
    renderer::{SpriteRender, SpriteSheet},
};
use amethyst::{core::transform::Transform, renderer::Camera, GameData, SimpleState, StateData};
use std::cell::RefCell;

#[derive(Default)]
pub struct Load<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
    progress: RefCell<ProgressCounter>,
}

impl<'a, 'b> SimpleState for Load<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.insert(AssetStorage::<Level>::new());

        camera::init(data.world);
        components::init(data.world);

        self.progress = RefCell::new(ProgressCounter::new());
        level::init(data.world, &mut self.progress);

        texture::init(data.world, &mut self.progress);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress.borrow().complete() {
            Completion::Complete => Trans::Switch(Box::new(states::Game::default())),
            Completion::Failed => {
                println!("something bad happened");
                Trans::Quit
            }
            Completion::Loading => Trans::None,
        }
    }
}
