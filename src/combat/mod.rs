use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::component::*;
use crate::menu::CastSkillEvent;

pub struct CombatPlugin;

/// State indicating whether it's the character's turn yet and can act
/// InTurn: character's turn, skill UI visible
/// NotInTurn: after character's turn, skill UI invisible
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CombatState {
    InTurn,
    NotInTurn,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhoseTurn {
    Player,
    Enemy,
}

/// hp trigger, story event, etc
pub struct SpecialTriggerEvent;
pub struct GameOverEvent;

pub struct FightClearedEvent;
pub struct EnemyKilledEvent;

pub struct TurnStartEvent;
pub struct EnemyTurnStartEvent;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(CombatState::InTurn)
            .add_loopless_state(WhoseTurn::Player)
            .add_event::<TurnStartEvent>()
            .add_event::<EnemyTurnStartEvent>()
            .add_system_set(
                ConditionSet::new()
                    // .with_system(logic_calc)
                    // .with_system(self_eval)
                    .with_system(enemy_eval)
                    .into(),
            );
    }
}

fn _logic_calc(mut ev_castskill: EventReader<CastSkillEvent>, _skill_q: Query<Entity, With<Skill>>) {
    for _ in ev_castskill.iter() {}
    // info!("combat: logic_calc");
}

// NOTE: new mechanics
// enters White Out when taking lethal damage
// player is at negative health but doesn't die yet,
// and will die if they get attacked again (opens up pre-casting heal)
// leaves White Out when they're at positive health

/// calculates changes to player's stat
fn _self_eval() {
    // TODO: heal for basics
}
/// calculates changes to enemies' stat
/// FIXME: this fires before checking for context history
fn enemy_eval(
    mut enemy: Query<(&LabelName, &mut Health, &mut Block), (With<Enemy>, Without<Player>)>,
    skill_q: Query<
        (Entity, Option<&Block>, Option<&Damage>, Option<&Heal>),
        (With<Skill>, Without<Player>, Without<Enemy>),
    >, // basic first
    mut ev_castskill: EventReader<CastSkillEvent>,
) {
    let (enemy_name, mut enemy_health, mut enemy_block) = enemy
        .get_single_mut()
        .expect("Should only have 1 enemy (for now)");

    for ev in ev_castskill.iter() {
        for (skill_ent, block, damage, heal) in skill_q.iter() {
            if skill_ent == ev.skill_ent.0 {
                if damage.is_some() {
                    let bleed_through = match block {
                        Some(block) => {
                            enemy_block.value -= damage.unwrap().value;
                            damage.unwrap().value - block.value
                        },
                        None => damage.unwrap().value
                    };
                    enemy_health.value -= bleed_through;
                    info!(
                        "enemy {} now has {} hp",
                        enemy_name.name, enemy_health.value
                    );
                }
                if enemy_health.value <= 0 {
                    // TODO: fires EnemyKilledEvent
                    info!("yay you won");
                }
            }
        }
    }
}

fn _is_all_enemies_dead(query: Query<&Enemy>) -> bool {
    query.is_empty()
}
