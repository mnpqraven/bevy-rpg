use crate::game::component::*;
use bevy::prelude::*;

use super::EnemyKilledEvent;
use super::EvalChannelingSkillEvent;
use super::EvalSkillEvent;
use super::TurnEndEvent;
use super::WhiteOut;

/// calculates changes to an unit's stat
/// TODO: implement for player + Target component to modularize
pub fn eval_instant_skill(
    mut target: Query<
        (Entity, &LabelName, &mut Health, &mut Block, Option<&Player>),
        Without<Skill>,
    >,
    skill_q: Query<
        (
            Entity,
            Option<&Block>,
            Option<&Damage>,
            Option<&Heal>,
        ),
        (With<Skill>, Without<Player>, Without<Enemy>),
    >,
    mut ev_evalskill: EventReader<EvalSkillEvent>,
    mut ev_enemykilled: EventWriter<EnemyKilledEvent>,
    mut commands: Commands,
    mut ev_endturn: EventWriter<TurnEndEvent>,
) {
    for ev in ev_evalskill.iter() {
        let (target_ent, target_name, mut target_health, mut target_block, target_player_tag) =
            target.get_mut(ev.target).unwrap();
        for (skill_ent, block, damage, heal) in skill_q.iter() {
            if skill_ent == ev.skill {
                if let Some(block) = block {
                    target_block.0 += block.0;
                    info!("Unit {}; Block {}", target_name.0, target_block.0);
                }
                if let Some(damage) = damage {
                    let bleed_through = match damage.0 > target_block.0 {
                        true => damage.0 - target_block.0,
                        false => 0,
                    };
                    target_health.0 -= bleed_through;
                    info!(
                        "target {} now has {} hp {} - {}",
                        target_name.0, target_health.0, damage.0, target_block.0
                    );
                }
                // add casting logic check
                if let Some(heal) = heal {
                    info!(
                        "target {} healing {} hp + {}",
                        target_name.0, target_health.0, heal.0
                    );
                    target_health.0 += heal.0;
                }
                if target_health.0 <= 0 {
                    match target_player_tag {
                        Some(_) => {
                            // EnterWhiteOutEvent
                            commands.entity(target_ent).insert(WhiteOut);
                        }
                        None => ev_enemykilled.send(EnemyKilledEvent(target_ent)),
                    }
                } else {
                    ev_endturn.send(TurnEndEvent);
                }
            }
        }
    }
}

/// channeling eval
pub fn eval_channeling_skill(
    mut ev_evalskill: EventReader<EvalChannelingSkillEvent>,
    mut ev_endturn: EventWriter<TurnEndEvent>,
) {
    for _ in ev_evalskill.iter() {
        ev_endturn.send(TurnEndEvent);
    }
}
