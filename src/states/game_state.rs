use crate::components::{Enemy, FireRate, Missile, Path, Speed, Tower, Velocity, CircleBounds, Health, Damage};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::systems::{MissileTargetter, TowerFirer};
use crate::{camera, texture};
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
};
use amethyst::{core::transform::Transform, renderer::Camera, GameData, SimpleState, StateData};

#[derive(Default)]
pub struct GameState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        camera::init(data.world);
        components::init(data.world);
        let sprite_sheet = texture::init(data.world);

        let mut dispatcher_builder = DispatcherBuilder::new()
            .with(systems::EnemyPather, "enemy_pather", &[])
            .with(systems::VelocityMover, "enemy_mover", &["enemy_pather"])
            .with(systems::PathDebugDraw, "debug_path_draw", &[])
            .with(
                systems::TowerRadiusDebugDraw,
                "tower_radius_debug_draw",
                &[],
            )
            .with(systems::EnemyInRangeTagger, "enemy_in_range_tagger", &[])
            .with(
                TowerFirer {
                    sprite_sheet: sprite_sheet.clone(),
                },
                "tower_firer",
                &["enemy_in_range_tagger"],
            )
            .with(
                systems::MissileTargetter,
                "missile_targetter",
                &["tower_firer"],
            )
            .with(systems::EnemyMissileCollider, "enemy_missile_collder", &[]);

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(data.world);

        self.dispatcher = Some(dispatcher);

        // Create single enemy for testing
        data.world
            .create_entity()
            .with(Enemy::default())
            .with(SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: 0,
            })
            .with({
                let mut transform = Transform::default();
                transform.set_translation_xyz(150.0, 150.0, 0.0);
                transform
            })
            .with(Path::new(vec![
                Point2::new(100.0, 100.0),
                Point2::new(200.0, 100.0),
                Point2::new(100.0, 200.0),
                Point2::new(200.0, 200.0),
                Point2::new(223.0, 242.0),
                Point2::new(254.0, 123.0),
            ]))
            .with(Velocity::default())
            .with(Speed(42.0))
            .with(CircleBounds {
                radius: 18.0,
            })
            .with(Health(100.0))
            .build();

        data.world
            .create_entity()
            .with(Tower { range: 150.0 })
            .with(SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: 1,
            })
            .with({
                let mut transform = Transform::default();
                transform.set_translation_xyz(200.0, 300.0, 0.0);
                transform
            })
            .with(FireRate::new(0.25))
            .build();

        // data.world
        //     .create_entity()
        //     .with(Missile {})
        //     .with(SpriteRender {
        //         sprite_sheet: sprite_sheet.clone(),
        //         sprite_number: 2,
        //     })
        //     .with({
        //         let mut transform = Transform::default();
        //         transform.set_translation_xyz(350.0, 350.0, 0.0);
        //         transform
        //     })
        //     .build();
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}
