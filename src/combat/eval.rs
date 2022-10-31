use crate::game::component::*;
use bevy::prelude::*;

use super::EnemyKilledEvent;
use super::EvalSkillEvent;
use super::TurnEndEvent;
use super::WhiteOut;

/// calculates changes to an unit's stat
/// TODO: implement for player + Target component to modularize
pub fn eval_skill(
    mut unit: Query<(Entity, &LabelName, &mut Health, &mut Block, Option<&Player>), Without<Skill>>,
    skill_q: Query<
        (Entity, Option<&Block>, Option<&Damage>, Option<&Heal>),
        (With<Skill>, Without<Player>, Without<Enemy>),
    >,
    mut ev_evalskill: EventReader<EvalSkillEvent>,
    mut ev_enemykilled: EventWriter<EnemyKilledEvent>,
    mut commands: Commands,
    mut ev_endturn: EventWriter<TurnEndEvent>,
) {
    for ev in ev_evalskill.iter() {
        let (target_ent, target_name, mut target_health, mut target_block, target_player_tag) =
            unit.get_mut(ev.target).unwrap();
        for (skill_ent, block, damage, _heal) in skill_q.iter() {
            if skill_ent == ev.skill {
                if block.is_some() {
                    target_block.value += block.unwrap().value;
                    debug!(
                        "should see {} block {}",
                        target_name.name, target_block.value
                    );
                }
                if damage.is_some() {
                    let bleed_through = match damage.unwrap().value > target_block.value {
                        true => damage.unwrap().value - target_block.value,
                        false => 0,
                    };
                    target_health.value -= bleed_through;
                    info!(
                        "target {} now has {} hp",
                        target_name.name, target_health.value
                    );
                }
                if target_health.value <= 0 {
                    match target_player_tag {
                        Some(_) => {
                            // EnterWhiteOutEvent
                            commands.entity(target_ent).insert(WhiteOut);
                        }
                        none => ev_enemykilled.send(EnemyKilledEvent(target_ent)),
                    }
                } else {
                    ev_endturn.send(TurnEndEvent);
                }
            }
        }
    }
}
