use crate::components::{Enemy, Missile, Speed, Velocity};
use amethyst::{
    core::math::{Unit, Vector2},
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MissileTargetter;

impl<'s> System<'s> for MissileTargetter {
    type SystemData = (
        ReadStorage<'s, Missile>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (missiles, transforms, mut velocities): Self::SystemData) {
        for (missile, transform, velocity) in (&missiles, &transforms, &mut velocities).join() {
            match transforms.get(missile.enemy) {
                Some(enemy_transform) => {
                    let x = enemy_transform.translation().x - transform.translation().x;
                    let y = enemy_transform.translation().y - transform.translation().y;

                    if x == 0.0 && y == 0.0 {
                        velocity.0 = None
                    } else {
                        velocity.0 = Some(Unit::new_normalize(Vector2::new(x, y)))
                    }
                }
                None => (), // TODO delete missile entity
            }
        }
    }
}
