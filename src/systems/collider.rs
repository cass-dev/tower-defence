use crate::components::{CircleBounds, Damage, Enemy, Health, Missile};
use amethyst::{
    core::math,
    core::math::Point2,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

#[derive(SystemDesc)]
pub struct EnemyMissileCollider;

impl<'s> System<'s> for EnemyMissileCollider {
    type SystemData = (
        ReadStorage<'s, CircleBounds>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Missile>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Damage>,
        WriteStorage<'s, Health>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (circle_bounds, enemies, missiles, transforms, damages, mut healths, entities): Self::SystemData,
    ) {
        for (enemy_circle_bound, enemy_transform, _, mut health, enemy_entity) in (
            &circle_bounds,
            &transforms,
            &enemies,
            &mut healths,
            &*entities,
        )
            .join()
        {
            for (missile_circle_bound, missile_transform, _, damage, missile_entity) in
                (&circle_bounds, &transforms, &missiles, &damages, &*entities).join()
            {
                let enemy_position = enemy_transform.translation();
                let missile_position = missile_transform.translation();

                let distance = math::distance(
                    &Point2::new(enemy_position.x, enemy_position.y),
                    &Point2::new(missile_position.x, missile_position.y),
                );

                if distance < enemy_circle_bound.radius + missile_circle_bound.radius {
                    entities.delete(missile_entity);
                    health -= damage;

                    if health.0 <= 0.0 {
                        entities.delete(enemy_entity);
                    }
                }
            }
        }
    }
}
