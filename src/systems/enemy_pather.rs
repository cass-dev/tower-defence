use crate::components::{Enemy, Path, Velocity};
use amethyst::{
    core::math::{Point2, Point3, Unit, Vector2, Vector3},
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, Write, WriteStorage},
    renderer::{
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
    },
};

#[derive(SystemDesc)]
pub struct EnemyPather;

impl<'s> System<'s> for EnemyPather {
    type SystemData = (
        ReadStorage<'s, Path>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Velocity>,
        Write<'s, DebugLines>,
    );

    fn run(
        &mut self,
        (paths, mut transforms, mut enemies, mut velocities, mut debug_lines): Self::SystemData,
    ) {
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
                            let new_next_path = path.points[path_index + 2];

                            let angle = (new_next_path.y - next_point.y)
                                .atan2((new_next_path.x - next_point.x));

                            transform.set_rotation_z_axis(angle);

                            enemy.path_index = Some(path_index + 1);
                        }
                    }

                    velocity.0 = Some(Unit::new_normalize(new_velocity));
                }
                None => {
                    enemy.path_index = Some(0);
                    let current_point = path.points[0];
                    let next_point = path.points[1];

                    let angle =
                        (next_point.y - current_point.y).atan2((next_point.x - current_point.x));

                    transform.set_rotation_z_axis(angle);

                    transform.set_translation_xyz(current_point.x, current_point.y, 0.0);
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
