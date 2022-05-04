use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(InInventory)]
#[read_component(Name)]
#[read_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    // SubWorld is like World, but can only see the components that were requested

    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let (player, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();

                let mut items = <(Entity, &Item, &Point, &Name)>::query();
                items
                    .iter(ecs)
                    .filter(|(_entity, _item, &item_pos, _)| item_pos == player_pos)
                    .for_each(|(entity, _item, _item_pos, _item_name)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, InInventory(player));

                        println!("Player picked up: {}", _item_name.name);

                        if let Ok(e) = ecs.entry_ref(*entity) {
                            if e.get_component::<Weapon>().is_ok() {
                                <(Entity, &InInventory, &Weapon)>::query()
                                    .iter(ecs)
                                    .filter(|(_, i, _)| i.0 == player)
                                    .for_each(|(e, i, w)| {
                                        commands.remove(*e);
                                    })
                            }
                        }
                    });
                Point::new(0, 0)
            }
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, position)| Some((*entity, *position + delta)))
            .unwrap();

        let mut did_something = false;

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;

            // check for enemies
            enemies
                .iter(ecs)
                .filter(|(_, position)| **position == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            // no enemies there, want to move to the destination
            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}

fn use_item(item_index_to_use: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();

    // get all items, filter out those not in the player's inventory
    let item_entity = <(Entity, &Item, &InInventory)>::query()
        .iter(ecs)
        .filter(|(_, _, in_inventory)| in_inventory.0 == player_entity)
        .enumerate()
        .filter(|(item_number, (_, _, _))| *item_number == item_index_to_use)
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity));

    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }
    Point::zero()
}
