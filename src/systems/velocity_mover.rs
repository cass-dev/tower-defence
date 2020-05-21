use crate::components::{Enemy, Path, Speed, Velocity};
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

#[derive(SystemDesc)]
pub struct VelocityMover;

impl<'s> System<'s> for VelocityMover {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, Speed>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, speeds, time): Self::SystemData) {
        for (transform, velocity, speed) in (&mut transforms, &velocities, &speeds).join() {
            match velocity.0 {
                Some(velocity) => {
                    transform.prepend_translation_x(velocity.x * time.delta_seconds() * speed);
                    transform.prepend_translation_y(velocity.y * time.delta_seconds() * speed);
                }
                None => {}
            }
        }
    }
}
