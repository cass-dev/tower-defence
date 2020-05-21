pub use self::enemy::Enemy;
pub use self::fire_rate::FireRate;
pub use self::in_range::InRange;
pub use self::missile::Missile;
pub use self::path::Path;
pub use self::speed::Speed;
pub use self::tower::Tower;
pub use self::velocity::Velocity;
pub use self::bounds::CircleBounds;
pub use self::health::Health;
pub use self::damage::Damage;

use amethyst::prelude::{World, WorldExt};

mod enemy;
mod fire_rate;
mod in_range;
mod missile;
mod path;
mod speed;
mod tower;
mod velocity;
mod bounds;
mod health;
mod damage;

pub fn init(world: &mut World) {
    world.register::<Enemy>();
    world.register::<Path>();
    world.register::<Velocity>();
    world.register::<Speed>();
    world.register::<Tower>();
    world.register::<InRange>();
    world.register::<Missile>();
    world.register::<FireRate>();
    world.register::<CircleBounds>();
    world.register::<Health>();
    world.register::<Damage>();
}
