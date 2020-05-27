mod speed;
mod system;
mod velocity;

pub use self::speed::Speed;
pub use self::velocity::Velocity;

use crate::physics::system::System;
use amethyst::ecs::DispatcherBuilder;
use amethyst::prelude::{World, WorldExt};

pub fn init<'a, 'b>(
    world: &mut World,
    mut dispatcher_builder: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    world.register::<Speed>();
    world.register::<Velocity>();

    dispatcher_builder.with(System, "physics_system", &[])
}
