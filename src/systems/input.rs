use legion::*;
use rltk::{ Rltk, VirtualKeyCode };

use crate::State;
use crate::components::labels::Player;
use crate::components::position::Position;
use crate::components::velocity::Velocity;

pub fn try_move(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut query = <(&Player, &Position, &mut Velocity)>::query();

    for (_player, position, velocity) in query.iter_mut(world) {
        if position.x + delta_x <= 0 {
            return;
        }

        if position.x + delta_x >= 79 {
            return;
        }

        if position.y + delta_y <= 0 {
            return;
        }

        if position.y + delta_y >= 49 {
            return;
        }

        velocity.x += delta_x;
        velocity.y += delta_y;
    }
}

pub fn process(gamestate: &mut State, context: &mut Rltk) {
    match context.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::A => try_move(-1, 0, &mut gamestate.world),
            VirtualKeyCode::D => try_move(1, 0, &mut gamestate.world),
            VirtualKeyCode::W => try_move(0, -1, &mut gamestate.world),
            VirtualKeyCode::S => try_move(0, 1, &mut gamestate.world),
            _ => {}
        }
    }
}
