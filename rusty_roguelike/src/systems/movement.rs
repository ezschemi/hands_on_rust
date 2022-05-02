use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        // access the playe entity, but outside a query. The specified component
        // needs to be declared first with read_component() or write_component()
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            // entity exists and it is the player
            camera.on_player_move(want_move.destination);
        }

        // remove this message, so it does not get processed again
        commands.remove(*entity);
    }
}
