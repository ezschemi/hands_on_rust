mod collisions;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    // flush() is a barrier between the collision detection and the rest.
    // it ensures that the collision detection is done, any entities that
    // collided are removed, and only the proceedes
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build()
}