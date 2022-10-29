pub mod ai;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game::component::*, menu::{despawn_with, spawn_skill_buttons, GameState}};

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
    System,
}

/// queue who's the next in turn
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NextInTurn {
    Player,
    Enemy,
}

/// hp trigger, story event, etc
pub struct SpecialTriggerEvent;
pub struct GameOverEvent;

pub struct FightClearedEvent;
pub struct EnemyKilledEvent(Entity);

pub struct TurnStartEvent;
pub struct EnemyTurnStartEvent;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(CombatState::InTurn)
        .add_loopless_state(WhoseTurn::Player)
        .add_loopless_state(NextInTurn::Enemy)
        // Player turn start
        .add_event::<TurnStartEvent>()
        .add_enter_system_set(WhoseTurn::Player,
            ConditionSet::new()
            .with_system(ev_player_turn_start)
            .with_system(spawn_skill_buttons.run_in_state(GameState::InCombat))
            .into()
            )
        .add_exit_system(WhoseTurn::Player, despawn_with::<SkillIcon>)
        // Enemy turn start
        .add_event::<EnemyTurnStartEvent>()
        .add_enter_system(WhoseTurn::Enemy, ev_enemy_turn_start)
        // system - stat eval
        .add_enter_system_set(
            WhoseTurn::System,
            ConditionSet::new()
            .label("eval")
            .with_system(logic_calc)
            .with_system(eval_self)
            // TODO: link this and eval clean up
            .with_system(eval_enemy)
            .into(),
        )
        // system - turn eval
        .add_system_set(
            ConditionSet::new()
            .label("eval2")
            .after("eval")
            // eval done, now solving turn
            .with_system(ev_reward.run_if(is_all_enemies_dead))
            .into()
        )
        // EnemyKilled
        .add_event::<EnemyKilledEvent>()
        .add_system(ev_enemykilled)
        // enter/exit state checks ?

        ;
    }
}

fn ev_reward() {}

fn ev_enemy_turn_start(
    // TODO: refactor to other chunks later
    mut ev_castskill: EventWriter<CastSkillEvent>,
    enemy_skill_q: Query<(Entity, &LabelName, &SkillGroup), With<SkillGroup>>,
) {
    debug!("WhoseTurn::Enemy");
    // only enemy skills rn, expand to universal later when we restructure skill data
    for (enemy_skill_ent, enemy_skill_name, _skill_group) in enemy_skill_q.iter().filter(| ( _, _, grp) | **grp == SkillGroup::Enemy) { // double deref ??
        // FIXME: make this CastSkillEvent doesn't listen
        info!("enemy casting {}", enemy_skill_name.name);
        ev_castskill.send(CastSkillEvent { skill_ent: SkillEnt(enemy_skill_ent) })
    }
}

fn ev_player_turn_start() {
    debug!("WhoseTurn::Player");
}
fn logic_calc(
    mut ev_castskill: EventReader<CastSkillEvent>,
    _skill_q: Query<Entity, With<Skill>>,
) {
    for _ in ev_castskill.iter() {}
    info!("combat: logic_calc");
}

// NOTE: new mechanics
// enters White Out when taking lethal damage
// player is at negative health but doesn't die yet,
// and will die if they get attacked again (opens up pre-casting heal)
// leaves White Out when they're at positive health
#[derive(Component)]
pub struct WhiteOut;

/// calculates changes to player's stat
fn eval_self() {
    // TODO: heal for basics
}
/// calculates changes to enemies' stat
fn eval_enemy(
    mut enemy: Query<(Entity, &LabelName, &mut Health, &mut Block), (With<Enemy>, Without<Player>)>,
    skill_q: Query<
        (Entity, Option<&Block>, Option<&Damage>, Option<&Heal>),
        (With<Skill>, Without<Player>, Without<Enemy>),
    >, // basic first
    mut ev_castskill: EventReader<CastSkillEvent>,
    mut ev_enemykilled: EventWriter<EnemyKilledEvent>,
    next_in_turn: Res<CurrentState<NextInTurn>>,
    mut commands: Commands
) {
    let (enemy_ent, enemy_name, mut enemy_health, mut enemy_block) = enemy
        .get_single_mut()
        .expect("Should only have 1 enemy (for now)");

    for ev in ev_castskill.iter() {
        for (skill_ent, block, damage, _heal) in skill_q.iter() {
            if skill_ent == ev.skill_ent.0 {
                if damage.is_some() {
                    let bleed_through = match block {
                        Some(block) => {
                            enemy_block.value -= damage.unwrap().value;
                            damage.unwrap().value - block.value
                        }
                        None => damage.unwrap().value,
                    };
                    enemy_health.value -= bleed_through;
                    info!(
                        "enemy {} now has {} hp",
                        enemy_name.name, enemy_health.value
                    );
                }
                if enemy_health.value <= 0 {
                    ev_enemykilled.send(EnemyKilledEvent(enemy_ent));
                } else {
                    // eval done, update next turn state and queue
                    match next_in_turn.0 {
                        NextInTurn::Player => {
                        commands.insert_resource(NextState(NextInTurn::Enemy));
                        commands.insert_resource(NextState(WhoseTurn::Player));
                        }
                        NextInTurn::Enemy => {
                        commands.insert_resource(NextState(NextInTurn::Player));
                        commands.insert_resource(NextState(WhoseTurn::Enemy));
                        }
                    }
                }
            }
        }
    }
}

/// event triggers when an enemy dies
fn ev_enemykilled(mut ev_enemykilled: EventReader<EnemyKilledEvent>, mut commands: Commands) {
    for ev in ev_enemykilled.iter() {
        info!("{:?} slain", ev.0);
        commands.entity(ev.0).despawn();
    }
}
fn is_all_enemies_dead(query: Query<&Enemy>) -> bool {
    query.is_empty()
}
