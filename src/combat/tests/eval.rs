#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;
    use bevy::prelude::*;

    use crate::combat::eval::eval_instant_skill;
    use crate::combat::{EvalSkillEvent, UIBarChangeEvent, UnitKilledEvent};
    use crate::ecs::component::{Ally, Block, Damage, Enemy, Health, Skill};

    #[test]
    fn damage_calculation() {
        let mut app = App::new();

        // setup world
        app.add_event::<UnitKilledEvent>();
        app.add_event::<EvalSkillEvent>();
        app.add_event::<UIBarChangeEvent>();
        app.add_system(eval_instant_skill);

        // setup entities
        let should_die_enemy = app.world.spawn((Enemy, Health(1), Block(3))).id();
        let blocked_live_enemy = app.world.spawn((Enemy, Health(1), Block(12))).id();
        let bleedthrough_live_enemy = app.world.spawn((Enemy, Health(8), Block(4))).id();
        let ally_id = app.world.spawn((Ally, Health(1))).id();
        let skill_id = app.world.spawn((Skill, Damage(10))).id();
        app.world.send_event(EvalSkillEvent {
            skill: skill_id,
            target: should_die_enemy,
            caster: ally_id,
        });
        app.world.send_event(EvalSkillEvent {
            skill: skill_id,
            target: blocked_live_enemy,
            caster: ally_id,
        });
        app.world.send_event(EvalSkillEvent {
            skill: skill_id,
            target: bleedthrough_live_enemy,
            caster: ally_id,
        });
        // run system
        app.update();

        // Check enemy was despawned
        // assert!(app.world.get::<Enemy>(enemy_id).is_none());
        assert_eq!(app.world.get::<Health>(should_die_enemy).unwrap().0, -6);
        assert_eq!(app.world.get::<Block>(should_die_enemy).unwrap().0, 0);
        assert_eq!(app.world.get::<Health>(blocked_live_enemy).unwrap().0, 1);
        assert_eq!(app.world.get::<Block>(blocked_live_enemy).unwrap().0, 2);
        assert_eq!(app.world.get::<Health>(bleedthrough_live_enemy).unwrap().0, 2);
        assert_eq!(app.world.get::<Block>(bleedthrough_live_enemy).unwrap().0, 0);
        app.update();

        // Get `UnitKilledEvent` event reader
        let unitkilled_event = app.world.resource::<Events<UnitKilledEvent>>();
        let mut evread_unitkilled = unitkilled_event.get_reader();
        let enemy_died = evread_unitkilled.iter(unitkilled_event).next().unwrap();

        // Check the event has been sent
        assert_eq!(enemy_died.0, should_die_enemy);
    }
}
