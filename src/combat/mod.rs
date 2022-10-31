mod ai;
mod eval;
use self::eval::eval_skill;
use crate::game::component::*;
use crate::game::despawn_with;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct CombatPlugin;
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

pub struct TurnEndEvent;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(WhoseTurn::Player)
            .add_loopless_state(NextInTurn::Enemy)
            // Player turn start
            .add_event::<TurnStartEvent>()
            .add_event::<EnemyTurnStartEvent>()
            .add_event::<EnterWhiteOutEvent>()
            // ----
            .add_event::<EnemyKilledEvent>()
            .add_system(ev_enemykilled)
            // ----
            .add_event::<CastSkillEvent>()
            .add_system(ev_watch_castskill)
            .add_event::<EvalSkillEvent>()
            // ----
            .add_event::<TurnEndEvent>()
            .add_system(evread_endturn)
            // ----
            .add_enter_system(WhoseTurn::Player, ev_player_turn_start)
            .add_exit_system(WhoseTurn::Player, despawn_with::<SkillIcon>)
            // ----
            .add_enter_system(WhoseTurn::Enemy, ev_enemy_turn_start)
            // ----
            .add_enter_system_set(
                WhoseTurn::System,
                ConditionSet::new().with_system(eval_skill).into(),
            );
    }
}

fn ev_reward() {}

fn ev_enemy_turn_start(
    // TODO: refactor to other chunks later
    player: Query<Entity, With<Player>>,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    enemy_skill_q: Query<(Entity, &SkillGroup), With<SkillGroup>>,
) {
    debug!("WhoseTurn::Enemy");
    // only enemy skills rn, expand to universal later when we restructure skill data
    for (enemy_skill_ent, _) in enemy_skill_q
        .iter()
        .filter(|(_, grp)| **grp == SkillGroup::Enemy)
    {
        ev_castskill.send(CastSkillEvent {
            skill_ent: SkillEnt(enemy_skill_ent),
            target: player.single(),
        });
    }
}

/// watch for SkillCastEvent from other modules
/// this will handle targetting for now
fn ev_watch_castskill(
    mut commands: Commands,
    mut ev_castskill: EventReader<CastSkillEvent>,
    skill_q: Query<(Entity, &LabelName, &Target), With<Skill>>,
    mut ev_skilltoeval: EventWriter<EvalSkillEvent>,
) {
    for ev in ev_castskill.iter() {
        for (skill_ent, skill_name, skill_target) in
            skill_q.iter().filter(|ent| ent.0 == ev.skill_ent.0)
        {
            info!(
                "CastSkillEvent {:?} {:?} {:?}",
                skill_ent, skill_name.name, skill_target
            );
        }
        commands.insert_resource(NextState(WhoseTurn::System));
        ev_skilltoeval.send(EvalSkillEvent {
            skill: ev.skill_ent.0,
            target: ev.target,
        });
    }
}

pub struct EvalSkillEvent {
    skill: Entity,
    target: Entity,
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

fn evread_endturn(
    mut commands: Commands,
    mut ev_endturn: EventReader<TurnEndEvent>,
    next_in_turn: Res<CurrentState<NextInTurn>>,
) {
    for _ in ev_endturn.iter() {
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
/// event triggers when an enemy dies
fn ev_enemykilled(mut ev_enemykilled: EventReader<EnemyKilledEvent>, mut commands: Commands) {
    for ev in ev_enemykilled.iter() {
        info!("{:?} slain", ev.0);
        commands.entity(ev.0).despawn();
    }
}
