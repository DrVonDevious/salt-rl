use rltk::Rltk;
use legion::*;

use crate::components::position::Position;
use crate::components::renderable::Renderable;

pub fn process(world: &World, context: &mut Rltk) {
    let mut query = <(&Position, &Renderable)>::query();

    for (position, renderable) in query.iter(world) {
        context.set(
            position.x,
            position.y,
            renderable.fg,
            renderable.bg,
            renderable.glyph,
        );
    }
}
