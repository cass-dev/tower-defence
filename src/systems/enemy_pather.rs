use crate::components::{Enemy, PathFollower, PathingState, Velocity};
use crate::resources::Path;
use amethyst::{
    core::math::{distance, Point2, Point3, Unit, Vector2, Vector3},
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
        Read<'s, Path>,
        WriteStorage<'s, PathFollower>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
    );

    fn run(
        &mut self,
        (path, mut path_followers, mut transforms, mut velocities): Self::SystemData,
    ) {
        for (mut transform, mut path_follower, mut velocity) in
            (&mut transforms, &mut path_followers, &mut velocities).join()
        {
            match path_follower.pathing_state {
                PathingState::SnapToPoint(path_index) => {
                    let point = path[path_index];
                    transform.set_translation_x(point.x);
                    transform.set_translation_y(point.y);

                    let next_point = (path_index + 1) % path.len();
                    path_follower.pathing_state = PathingState::MoveToPoint(next_point, point)
                }
                PathingState::MoveToPoint(path_index, start_position) => {
                    let current_position = transform.translation().clone();
                    let next_position = path[path_index];
                    let angle = (next_position.y - current_position.y)
                        .atan2((next_position.x - current_position.x));
                    transform.set_rotation_z_axis(angle);

                    let new_velocity = Vector2::new(
                        next_position.x - current_position.x,
                        next_position.y - current_position.y,
                    );
                    velocity.0 = Some(Unit::new_normalize(new_velocity));

                    if has_passed_point(&next_position, &current_position, &start_position) {
                        transform.set_translation_x(next_position.x);
                        transform.set_translation_y(next_position.y);

                        let next_point = (path_index + 1) % path.len();
                        if next_point == 0 {
                            path_follower.pathing_state = PathingState::SnapToPoint(next_point)
                        } else {
                            path_follower.pathing_state =
                                PathingState::MoveToPoint(next_point, next_position)
                        }
                    }
                }
            }
        }
    }
}

fn has_passed_point(
    end: &Point2<f32>,
    current_position: &Vector3<f32>,
    start_position: &Point2<f32>,
) -> bool {
    distance(start_position, end)
        < distance(
            start_position,
            &Point2::new(current_position.x, current_position.y),
        )
}
