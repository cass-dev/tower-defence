use crate::components::{Enemy, InRange, Path, Speed, Tower, Velocity};
use amethyst::{
    core::math,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

#[derive(SystemDesc)]
pub struct EnemyInRangeTagger;

impl<'s> System<'s> for EnemyInRangeTagger {
    type SystemData = (
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Tower>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, InRange>,
        Entities<'s>,
    );

    fn run(&mut self, (enemies, towers, transforms, mut inRanges, entities): Self::SystemData) {
        for (_, enemy_transform, enemy_entity) in (&enemies, &transforms, &*entities).join() {
            for (tower, tower_transform, tower_entity) in (&towers, &transforms, &*entities).join()
            {
                let distance = math::distance(
                    &math::Point2::new(
                        enemy_transform.translation().x,
                        enemy_transform.translation().y,
                    ),
                    &math::Point2::new(
                        tower_transform.translation().x,
                        tower_transform.translation().y,
                    ),
                );
                if distance < tower.range {
                    inRanges.insert(
                        tower_entity,
                        InRange {
                            other: enemy_entity,
                        },
                    );
                    inRanges.insert(
                        enemy_entity,
                        InRange {
                            other: tower_entity,
                        },
                    );
                } else {
                    inRanges.remove(tower_entity);
                    inRanges.remove(enemy_entity);
                }
            }
        }
    }
}
