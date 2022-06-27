use rltk::{ Rltk, GameState, RGB };
use legion::*;

mod components;
mod systems;

use components::renderable::Renderable;
use components::velocity::Velocity;
use components::position::Position;
use components::labels::Player;

pub struct State {
    world: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Welcome To Salt");

        systems::render::process(&self.world, ctx);
        systems::movement::process(&mut self.world);
        systems::input::process(self, ctx);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Salt")
        .build()?;

    let mut gamestate = State{
        world: World::default(),
    };

    gamestate.world.push((
        Position { x: 40, y: 25 },
        Velocity { x: 0, y: 0 },
        Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        },
        Player {},
    ));

    rltk::main_loop(context, gamestate)
}
