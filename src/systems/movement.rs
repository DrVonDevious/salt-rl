use legion::*;

use crate::components::position::Position;
use crate::components::velocity::Velocity;

pub fn process(world: &mut World) {
    let mut query = <(&mut Position, &mut Velocity)>::query();

    for (position, velocity) in query.iter_mut(world) {
        position.x += velocity.x;
        position.y += velocity.y;
        
        if velocity.x > 0 {
            velocity.x -= 1;
        }

        if velocity.x < 0 {
            velocity.x += 1;
        }

        if velocity.y > 0 {
            velocity.y -= 1;
        }

        if velocity.y < 0 {
            velocity.y += 1;
        }
    }
}
