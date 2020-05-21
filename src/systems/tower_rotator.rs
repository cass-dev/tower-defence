use crate::components::{
    CircleBounds, Damage, Enemy, FireRate, InRange, Missile, Speed, Tower, Velocity,
};
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
pub struct TowerRotator;

impl<'s> System<'s> for TowerRotator {
    type SystemData = (
        ReadStorage<'s, Tower>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, InRange>,
    );

    fn run(&mut self, (towers, mut transforms, in_ranges): Self::SystemData) {
        for (_, tower_transform, in_range) in (&towers, &mut transforms, &in_ranges).join() {
            let angle =
                (in_range.other_transform.translation().y - tower_transform.translation().y).atan2(
                    (in_range.other_transform.translation().x - tower_transform.translation().x),
                );

            tower_transform.set_rotation_z_axis(angle - (0.5 * 3.142));
        }
    }
}
