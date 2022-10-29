pub mod ai;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    game::component::*,
    menu::{despawn_with, draw_skill_icons, GameState},
};

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
            .add_event::<EnterWhiteOutEvent>()
            .add_enter_system_set(
                WhoseTurn::Player,
                ConditionSet::new()
                    .with_system(ev_player_turn_start)
                    .with_system(draw_skill_icons.run_in_state(GameState::InCombat))
                    .into(),
            )
            .add_exit_system(WhoseTurn::Player, despawn_with::<SkillIcon>)
            // Enemy turn start
            .add_event::<EnemyTurnStartEvent>()
            .add_enter_system(WhoseTurn::Enemy, ev_enemy_turn_start)
            // system - stat eval
            .add_enter_system_set(
                WhoseTurn::System,
                ConditionSet::new()
                    .label("system_eval")
                    .with_system(eval_skill)
                    .into(),
            )
            // system - turn eval
            .add_system_set(
                ConditionSet::new()
                    .label("reward_eval")
                    .after("system_eval")
                    // eval done, now solving turn
                    .with_system(ev_reward.run_if(is_all_enemies_dead))
                    .into(),
            )
            .add_event::<EnemyKilledEvent>()
            .add_system(ev_enemykilled)
            .add_system(ev_castskill.run_not_in_state(WhoseTurn::System));
    }
}

fn ev_reward() {}

fn ev_enemy_turn_start(
    // TODO: refactor to other chunks later
    player: Query<Entity, With<Player>>,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    enemy_skill_q: Query<(Entity, &LabelName, &SkillGroup), With<SkillGroup>>,
) {
    debug!("WhoseTurn::Enemy");
    // only enemy skills rn, expand to universal later when we restructure skill data
    for (enemy_skill_ent, enemy_skill_name, _) in enemy_skill_q
        .iter()
        .filter(|(_, _, grp)| **grp == SkillGroup::Enemy)
    {
        info!(
            "CastSkillEvent {:?} {}",
            enemy_skill_ent, enemy_skill_name.name
        );
        // WARN: update WhoseTurn here with commands.insert_resource
        // will cause infinite loop
        ev_castskill.send(CastSkillEvent {
            skill_ent: SkillEnt(enemy_skill_ent),
            target: player.single(),
        });
    }
}

fn ev_castskill(mut commands: Commands, mut ev_castskill: EventReader<CastSkillEvent>) {
    // TODO: this works but think of a better way to use this instead of an extra method
    // updates turn state when skill is cast, system handles evaluating skills
    for _ in ev_castskill.iter() {
        commands.insert_resource(NextState(WhoseTurn::System));
    }
}

fn ev_player_turn_start() {
    debug!("WhoseTurn::Player");
}

// NOTE: new mechanics
// enters White Out when taking lethal damage
// player is at negative health but doesn't die yet,
// and will die if they get attacked again (opens up pre-casting heal)
// leaves White Out when they're at positive health
#[derive(Component)]
pub struct WhiteOut;
pub struct EnterWhiteOutEvent(Entity);

/// calculates changes to an unit's stat
/// TODO: implement for player + Target component to modularize
fn eval_skill(
    mut unit: Query<(Entity, &LabelName, &mut Health, &mut Block, Option<&Player>), Without<Skill>>,
    skill_q: Query<
        (Entity, Option<&Block>, Option<&Damage>, Option<&Heal>),
        (With<Skill>, Without<Player>, Without<Enemy>),
    >, // basic first
    mut ev_castskill: EventReader<CastSkillEvent>,
    mut ev_enemykilled: EventWriter<EnemyKilledEvent>,
    next_in_turn: Res<CurrentState<NextInTurn>>,
    mut commands: Commands,
) {
    info!("calculating..");
    for ev in ev_castskill.iter() {
        let (target_ent, target_name, mut target_health, mut target_block, target_player_tag) =
            unit.get_mut(ev.target).unwrap();
        let target_is_ally = match target_player_tag {
            Some(_) => true,
            None => false,
        };
        for (skill_ent, block, damage, _heal) in skill_q.iter() {
            if skill_ent == ev.skill_ent.0 {
                if damage.is_some() {
                    let bleed_through = match block {
                        Some(block) => {
                            target_block.value -= damage.unwrap().value;
                            damage.unwrap().value - block.value
                        }
                        None => damage.unwrap().value,
                    };
                    target_health.value -= bleed_through;
                    info!(
                        "target {} now has {} hp",
                        target_name.name, target_health.value
                    );
                }
                if target_health.value <= 0 {
                    match target_is_ally {
                        true => {
                            // EnterWhiteOutEvent
                            commands.entity(target_ent).insert(WhiteOut);
                        }
                        false => ev_enemykilled.send(EnemyKilledEvent(target_ent)),
                    }
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
