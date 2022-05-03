use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]

pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    // find all the things that want to chase the player
    let mut chasers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();

    let mut positions = <(Entity, &Point, &Health)>::query();

    let mut player = <(&Point, &Player)>::query();

    let player_position = player.iter(ecs).nth(0).unwrap().0;
    let player_index = map_index(player_position.x, player_position.y);

    let search_targets = vec![player_index];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    chasers.iter(ecs).for_each(|(entity, pos, _, fov)| {
        if !fov.visible_tiles.contains(&player_position) {
            return;
        }

        let index = map_index(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, index, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_position);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_position
            };

            let mut attacked = false;

            positions
                .iter(ecs)
                .filter(|(_, target_position, _)| **target_position == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                    attacked = true;
                });
            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
