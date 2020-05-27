mod bounds;
mod damage;
mod enemy;
mod fire_rate;
mod health;
mod in_range;
mod missile;
mod path_follower;
mod tower;

pub use self::bounds::CircleBounds;
pub use self::damage::Damage;
pub use self::enemy::Enemy;
pub use self::fire_rate::FireRate;
pub use self::health::Health;
pub use self::in_range::InRange;
pub use self::missile::Missile;
pub use self::path_follower::{PathFollower, PathingState};
pub use self::tower::Tower;

use amethyst::prelude::{World, WorldExt};

pub fn init(world: &mut World) {
    world.register::<Enemy>();
    world.register::<Tower>();
    world.register::<InRange>();
    world.register::<Missile>();
    world.register::<FireRate>();
    world.register::<CircleBounds>();
    world.register::<Health>();
    world.register::<Damage>();
    world.register::<PathFollower>();
}
