use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(RevealsDungeonMap)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_effects_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    println!("Using a healing item: +{} hp", healing.amount);
                    healing_effects_to_apply.push((activate.used_by, healing.amount));
                }
                if let Ok(_map_revealer) = item.get_component::<RevealsDungeonMap>() {
                    println!("Using a map revealer...");
                    map.revealed_tiles.iter_mut().for_each(|tile| *tile = true);
                }
            }
            commands.remove(activate.item);
            commands.remove(*entity);
        });

    for heal in healing_effects_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}
