use crate::components::{
    CircleBounds, Damage, Enemy, FireRate, Health, Missile, PathFollower, PathingState, Tower,
};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::level::Level;
use crate::resources::Path;
use crate::systems::{MissileTargetter, TowerFirer};
use crate::texture::SpriteSheetHandle;
use crate::{camera, physics, resources, texture};
use crate::{components, systems};
use amethyst::ecs::{Dispatcher, DispatcherBuilder};
use amethyst::input::is_close_requested;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::{
    assets::AssetStorage,
    core::math::{Orthographic3, Point2},
    ecs::{prelude::Entities, LazyUpdate},
    prelude::*,
    renderer::camera::Projection,
    renderer::transparent::Transparent,
    utils::application_root_dir,
    window::ScreenDimensions,
};
use amethyst::{
    assets::Handle,
    core::math::Vector3,
    renderer::{SpriteRender, SpriteSheet},
};
use amethyst::{core::transform::Transform, renderer::Camera, GameData, SimpleState, StateData};
use std::rc::Rc;

#[derive(Default)]
pub struct Game<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for Game<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        camera::init(data.world);
        components::init(data.world);

        let sprite_sheet = data.world.read_resource::<SpriteSheetHandle>().0.clone();

        data.world.insert(resources::DebugToggle::default());

        let mut dispatcher_builder = DispatcherBuilder::new()
            .with(systems::EnemyPather, "enemy_pather", &[])
            .with(systems::DebugToggle, "debug_toggle", &[])
            .with(systems::PathDebugDraw, "debug_path_draw", &["debug_toggle"])
            .with(
                systems::TowerRadiusDebugDraw,
                "tower_radius_debug_draw",
                &["debug_toggle"],
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
            .with(systems::EnemyMissileCollider, "enemy_missile_collder", &[])
            .with(systems::TowerRotator, "tower_rotator", &[]);

        dispatcher_builder = physics::init(data.world, dispatcher_builder);

        let mut dispatcher = dispatcher_builder.build();

        dispatcher.setup(data.world);

        self.dispatcher = Some(dispatcher);

        create_level(data.world, sprite_sheet);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
}

fn create_level(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let level = {
        let level_handle = world.read_resource::<Handle<Level>>();
        let asset_storage = world.read_resource::<AssetStorage<Level>>();
        asset_storage.get(&level_handle).unwrap().clone()
    };

    for y in (0..level.tiles.height).rev() {
        for x in (0..level.tiles.width).rev() {
            let i = (y * level.tiles.width) + x;
            match level.tiles.indexes.get(i) {
                Some(index) => world
                    .create_entity()
                    .with(SpriteRender {
                        sprite_sheet: sprite_sheet.clone(),
                        sprite_number: *index,
                    })
                    .with({
                        let pos = (x as f32, (level.tiles.height - y - 1) as f32);

                        let mut transform = Transform::default();
                        transform.set_translation_xyz(
                            (pos.0 * 64.0) + 32.0,
                            (pos.1 * 64.0) + 32.0,
                            0.0,
                        );
                        transform
                    })
                    .build(),
                None => panic!("bad index {}", i),
            };
        }
    }

    for tower in level.towers {
        let pos = (
            (tower.position.0 as f32 * 64.0) + 32.0,
            (tower.position.1 as f32 * 64.0) + 32.0,
        );

        // Tower base
        world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: (7 * 23) + 19,
            })
            .with({
                let mut transform = Transform::default();
                transform.set_translation_xyz(pos.0, pos.1, 1.0);
                transform
            })
            .with(Transparent)
            .build();

        // Tower turret
        world
            .create_entity()
            .with(Tower { range: 160.0 })
            .with(SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: (10 * 23) + 19,
            })
            .with({
                let mut transform = Transform::default();
                transform.set_translation_xyz(pos.0, pos.1, 2.0);
                transform
            })
            .with(FireRate::new(0.5))
            .with(Transparent)
            .build();
    }

    world.insert(Path(
        level
            .path
            .points
            .into_iter()
            .map(|(x, y)| Point2::new(x * 64.0, y * 64.0))
            .collect(),
    ));

    for enemy in level.enemies {
        let pos = (
            (enemy.start_position.0 as f32 * 64.0) + 32.0,
            (enemy.start_position.1 as f32 * 64.0) + 32.0,
        );

        world
            .create_entity()
            .with(Enemy::default())
            .with(SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: (10 * 23) + 15,
            })
            .with({
                let mut transform = Transform::default();
                transform.set_translation_xyz(pos.0, pos.1, 1.0);
                transform
            })
            .with(physics::Velocity::default())
            .with(physics::Speed(42.0))
            .with(CircleBounds { radius: 18.0 })
            .with(Health(10000.0))
            .with(Transparent)
            .with(PathFollower {
                pathing_state: PathingState::MoveToPoint(0, Point2::new(pos.0, pos.1)),
            })
            .build();
    }
}
