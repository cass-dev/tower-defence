use crate::components::{Enemy, Path, Velocity, Tower};
use amethyst::{
    core::math::Point3,
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
pub struct PathDebugDraw;

impl<'s> System<'s> for PathDebugDraw {
    type SystemData = (ReadStorage<'s, Path>, Write<'s, DebugLines>);

    fn run(&mut self, (paths, mut debug_lines_resource): Self::SystemData) {
        for path in (&paths).join() {
            for i in 0..path.points.len() - 1 {
                let point_a = &path.points[i];
                let point_b = &path.points[i + 1];

                debug_lines_resource.draw_circle(
                    Point3::new(point_a.x, point_a.y, 0.0),
                    5.0,
                    10,
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                );

                debug_lines_resource.draw_circle(
                    Point3::new(point_b.x, point_b.y, 0.0),
                    5.0,
                    10,
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                );

                debug_lines_resource.draw_line(
                    Point3::new(point_a.x, point_a.y, 0.0),
                    Point3::new(point_b.x, point_b.y, 0.0),
                    Srgba::new(1.0, 1.0, 1.0, 1.0),
                );
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct TowerRadiusDebugDraw;

impl<'s> System<'s> for TowerRadiusDebugDraw {
    type SystemData = (
        ReadStorage<'s, Tower>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (towers, transforms, mut debug_lines_resource): Self::SystemData) {
        for (tower, transform) in (&towers, &transforms).join() {
            debug_lines_resource.draw_circle(
                Point3::new(transform.translation().x, transform.translation().y, 0.0),
                tower.range,
                25,
                Srgba::new(1.0, 1.0, 1.0, 1.0),
            );
        }
    }
}
