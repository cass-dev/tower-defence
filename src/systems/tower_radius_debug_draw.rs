use crate::components::{Enemy, Path, Tower, Velocity};
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
