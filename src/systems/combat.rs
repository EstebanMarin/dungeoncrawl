use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity)> = attackers // (1)
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim)) // (2)
        .collect(); // (3)

    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut victim_entry) = ecs.entry_mut(*victim) {
            if let Ok(health) = victim_entry.get_component_mut::<Health>() {
                health.current -= 1;
                if health.current < 1 {
                    commands.remove(*victim);
                }
            }
        }
        commands.remove(*message);
    });
}
