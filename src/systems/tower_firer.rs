use crate::components::{Enemy, FireRate, InRange, Missile, Path, Speed, Tower, Velocity, Damage, CircleBounds};
use amethyst::prelude::Builder;
use amethyst::{
    assets::Handle,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{
        prelude::{Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
        LazyUpdate,
    },
    renderer::{SpriteRender, SpriteSheet},
};

#[derive(SystemDesc)]
#[system_desc(name(TowerFirerDesc))]
pub struct TowerFirer {
    pub sprite_sheet: Handle<SpriteSheet>,
}

impl<'s> System<'s> for TowerFirer {
    type SystemData = (
        ReadStorage<'s, Tower>,
        ReadStorage<'s, InRange>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, FireRate>,
        Entities<'s>,
        Read<'s, Time>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            towers,
            in_ranges,
            mut transforms,
            mut fire_rates,
            entities,
            time,
            lazy_update
        ): Self::SystemData,
    ) {
        for (_, enemy_in_range, tower_transform, fire_rate) in
            (&towers, &in_ranges, &transforms, &mut fire_rates).join()
        {
            if fire_rate.can_fire(time.delta_seconds()) {
                lazy_update
                    .create_entity(&entities)
                    .with(Missile {
                        enemy: enemy_in_range.other,
                    })
                    .with(SpriteRender {
                        sprite_sheet: self.sprite_sheet.clone(),
                        sprite_number: 2,
                    })
                    .with({
                        let mut transform = Transform::default();
                        transform.set_translation_xyz(
                            tower_transform.translation().x,
                            tower_transform.translation().y,
                            0.0,
                        );
                        transform
                    })
                    .with(Velocity::default())
                    .with(Speed(100.0))
                    .with(CircleBounds {
                        radius: 2.5,
                    })
                    .with(Damage(10.0))
                    .build();
            }
        }
    }
}
