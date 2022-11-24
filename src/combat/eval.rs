use crate::ecs::component::*;
use bevy::prelude::*;

use super::EvalChannelingSkillEvent;
use super::EvalSkillEvent;
use super::TurnEndEvent;
use super::UnitKilledEvent;
use super::WhiteOut;

/// Run change functions to an unit's stat
/// inside ControlMutex::System
pub fn eval_instant_skill(
    mut target: Query<(Entity, &mut Health, &mut Block, Option<&Player>), Without<Skill>>,
    skill_q: Query<
        (Option<&Block>, Option<&Damage>, Option<&Heal>),
        (With<Skill>, Without<Player>, Without<Enemy>),
    >,
    mut ev_evalskill: EventReader<EvalSkillEvent>,
    mut ev_enemykilled: EventWriter<UnitKilledEvent>,
    mut commands: Commands,
) {
    info!("ControlMutex::System: eval_instant_skill");
    for ev in ev_evalskill.iter() {
        let (target_ent, mut target_health, mut target_block, target_player_tag) =
            target.get_mut(ev.target).expect("can't find target entity");
        let (skill_block, skill_damage, skill_heal) =
            skill_q.get(ev.skill).expect("can't find skill entity");
        let (target_health, target_block) = (&mut *target_health, &mut *target_block);
        // ------------
        // Eval
        eval_block(skill_block, target_block);
        eval_heal(skill_heal, target_health);
        eval_damage(skill_damage, target_health, target_block);
        // ------------
        // TODO: Post-eval
        match target_health.0 {
            x if x <= 0 && target_player_tag.is_some() => {
                commands.entity(target_ent).insert(WhiteOut);
            }
            x if x <= 0 => {
                ev_enemykilled.send(UnitKilledEvent(target_ent));
            }
            _ => {}
        }
    }
}

/// Evaluate channel skills
/// Run inside ControlMutex::System
pub fn eval_channeling_skill(
    mut ev_evalskill: EventReader<EvalChannelingSkillEvent>,
    mut ev_endturn: EventWriter<TurnEndEvent>,
) {
    info!("ControlMutex::System: eval_channeling_skill");
    for _ in ev_evalskill.iter() {
        ev_endturn.send(TurnEndEvent);
    }
}

// NOTE: return type ?
/// Updates the target's block according to the skill's block stat
pub fn eval_block(skill_block: Option<&Block>, mut target_block: &mut Block) {
    if let Some(skill_block) = skill_block {
        target_block.0 += skill_block.0;
        info!("eval: +-{} block", skill_block.0);
    }
}
/// Updates the target's health and block according to the skill's damage stat
pub fn eval_damage(
    skill_damage: Option<&Damage>,
    target_health: &mut Health,
    target_block: &mut Block,
) {
    if let Some(skill_damage) = skill_damage {
        match target_block.0 <= skill_damage.0 {
            true => {
                target_health.0 -= skill_damage.0 - target_block.0;
                target_block.0 = 0;
            }
            false => target_block.0 -= skill_damage.0,
        }
    }
}
/// Updates the target's health according to the skill's healing stat
pub fn eval_heal(skill_heal: Option<&Heal>, target_health: &mut Health) {
    if let Some(skill_heal) = skill_heal {
        target_health.0 += skill_heal.0;
    }
}
