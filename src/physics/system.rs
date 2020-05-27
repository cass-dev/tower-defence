use crate::physics::{Speed, Velocity};
use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System as AmethystSystem, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct System;

impl<'s> AmethystSystem<'s> for System {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (velocities, speeds, mut transforms, time): Self::SystemData) {
        for (velocity, speed, transform) in (&velocities, &speeds, &mut transforms).join() {
            match velocity {
                Velocity(Some(unit_velocity)) => {
                    transform.prepend_translation_x(unit_velocity.x * speed * time.delta_seconds());
                    transform.prepend_translation_y(unit_velocity.y * speed * time.delta_seconds());
                }
                _ => (),
            }
        }
    }
}
