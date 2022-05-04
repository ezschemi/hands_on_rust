use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(InInventory)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        // find all the weapons in the inventory and sum up their damage values
        let weapon_damage: i32 = <(&InInventory, &Damage)>::query()
            .iter(ecs)
            .filter(|(in_inventory, _)| in_inventory.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let final_damage = base_damage + weapon_damage;

        // only hit entities that actually have any health
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);

            println!("Doing {} + {} = {} damage.", base_damage, weapon_damage, final_damage);

            health.current -= final_damage;

            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
}
