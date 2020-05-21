use crate::components::{Enemy, Path, Velocity};
use amethyst::{
    core::math::{Point2, Unit, Vector2, Vector3},
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, Write, WriteStorage},
};

#[derive(SystemDesc)]
pub struct EnemyPather;

impl<'s> System<'s> for EnemyPather {
    type SystemData = (
        ReadStorage<'s, Path>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, (paths, mut transforms, mut enemies, mut velocities): Self::SystemData) {
        for (path, mut transform, mut enemy, mut velocity) in
            (&paths, &mut transforms, &mut enemies, &mut velocities).join()
        {
            match enemy.path_index {
                Some(path_index) => {
                    let previous_point = path.points[path_index];
                    let next_point = path.points[path_index + 1];

                    let new_velocity = Vector2::new(
                        next_point.x - previous_point.x,
                        next_point.y - previous_point.y,
                    );

                    if has_passed_point(next_point, &new_velocity, &transform.translation()) {
                        if path_index == path.points.len() - 2 {
                            enemy.path_index = None;
                        } else {
                            transform.set_translation_xyz(next_point.x, next_point.y, 0.0);
                            enemy.path_index = Some(path_index + 1);
                        }
                    }

                    velocity.0 = Some(Unit::new_normalize(new_velocity));
                }
                None => {
                    enemy.path_index = Some(0);
                    transform.set_translation_xyz(path.points[0].x, path.points[0].y, 0.0);
                }
            }
        }
    }
}

fn has_passed_point(
    end: Point2<f32>,
    velocity: &Vector2<f32>,
    current_position: &Vector3<f32>,
) -> bool {
    let passed_horiz = velocity.x == 0.0
        || (velocity.x > 0.0 && current_position.x > end.x)
        || (velocity.x < 0.0 && current_position.x <= end.x);
    let passed_vert = velocity.y == 0.0
        || (velocity.y > 0.0 && current_position.y > end.y)
        || (velocity.y < 0.0 && current_position.y <= end.y);
    passed_horiz && passed_vert
}
