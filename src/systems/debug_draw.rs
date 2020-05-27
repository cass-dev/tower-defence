use crate::components::{Enemy, Tower};
use crate::resources;
use crate::resources::Path;
use amethyst::{
    core::math::Point3,
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
    },
};

#[derive(SystemDesc)]
pub struct DebugToggle;
impl<'s> System<'s> for DebugToggle {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, resources::DebugToggle>,
    );

    fn run(&mut self, (input, mut debug_toggle): Self::SystemData) {
        let should_toggle_button = input.action_is_down("debug_toggle").unwrap_or(false);

        if should_toggle_button && debug_toggle.can_toggle {
            debug_toggle.should_show = !debug_toggle.should_show;
        }

        debug_toggle.can_toggle = !should_toggle_button;
    }
}

#[derive(SystemDesc)]
pub struct PathDebugDraw;

impl<'s> System<'s> for PathDebugDraw {
    type SystemData = (
        Read<'s, Path>,
        Write<'s, DebugLines>,
        Read<'s, resources::DebugToggle>,
    );

    fn run(&mut self, (path, mut debug_lines_resource, debug_toggle): Self::SystemData) {
        if debug_toggle.should_show {
            for i in 0..path.0.len() - 1 {
                let point_a = &path.0[i];
                let point_b = &path.0[i + 1];

                debug_lines_resource.draw_circle(
                    Point3::new(point_a.x, point_a.y, 0.0),
                    5.0,
                    10,
                    Srgba::new(1.0, 0.0, 0.0, 1.0),
                );

                debug_lines_resource.draw_circle(
                    Point3::new(point_b.x, point_b.y, 0.0),
                    5.0,
                    10,
                    Srgba::new(1.0, 0.0, 0.0, 1.0),
                );

                debug_lines_resource.draw_line(
                    Point3::new(point_a.x, point_a.y, 0.0),
                    Point3::new(point_b.x, point_b.y, 0.0),
                    Srgba::new(1.0, 0.0, 0.0, 1.0),
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
        Read<'s, resources::DebugToggle>,
    );

    fn run(
        &mut self,
        (towers, transforms, mut debug_lines_resource, debug_toggle): Self::SystemData,
    ) {
        if debug_toggle.should_show {
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
}
