use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::{
    core::transform::Transform, renderer::Camera as AmethystCamera, GameData, SimpleState,
    StateData,
};

pub fn init(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 100.0);

    world
        .create_entity()
        .with(AmethystCamera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}
