mod ai;
mod eval;
use self::eval::eval_channeling_skill;
use self::eval::eval_instant_skill;
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
            .add_event::<EvalChannelingSkillEvent>()
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
                ConditionSet::new()
                    .with_system(eval_instant_skill)
                    .with_system(eval_channeling_skill)
                    .into(),
            );
    }
}

fn ev_reward() {}

// TODO: modularize
fn ev_player_turn_start(
    mut commands: Commands,
    mut casting_ally_q: Query<(Entity, &mut Channel, &Casting), With<Player>>,
    skill_q: Query<(Option<&Damage>, Option<&Heal>), With<Skill>>,
    mut unit_q: Query<&mut Health>,
) {
    info!("WhoseTurn::Player");
    // handle channel
    for (unit_ent, mut unit_channel, unit_casting) in &mut casting_ally_q {
        match unit_channel.0 {
            1 => {
                commands.entity(unit_ent).remove::<Channel>();
                commands.entity(unit_ent).remove::<Casting>();
                debug!("{:?}", unit_casting.skill_ent);
                let (skill_damage, skill_heal) = skill_q.get(unit_casting.skill_ent).unwrap();
                let mut target_hp = unit_q.get_mut(unit_casting.target_ent).unwrap();
                if let Some(heal) = skill_heal {
                    target_hp.0 += heal.0;
                }
                if let Some(damage) = skill_damage {
                    // FIXME: damage already applied during skill press
                    debug!("{} {}", &target_hp.0, &damage.0);
                    target_hp.0 -= damage.0;
                }
                debug!("{:?}", &target_hp.0);
                // allow choosing skill
            }
            _ => {
                // TODO: skips player turn if casting
                unit_channel.0 -= 1;
                // commands.insert_resource(NextState(WhoseTurn::System));
            }
        }
    }
}
fn ev_enemy_turn_start(
    // TODO: refactor to other chunks later
    player: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    enemy_skill_q: Query<(Entity, &SkillGroup), With<Skill>>,
) {
    info!("WhoseTurn::Enemy");
    // only enemy skills rn, expand to universal later when we restructure skill data
    // TODO: fetch skill ent from enemy ai algorithm
    for (enemy_skill_ent, _) in enemy_skill_q
        .iter()
        .filter(|(_, grp)| **grp == SkillGroup::Enemy)
    {
        ev_castskill.send(CastSkillEvent {
            skill_ent: SkillEnt(enemy_skill_ent),
            target: player.single(),
            caster: enemies.iter().next().unwrap(),
        });
    }
}

/// watch for SkillCastEvent from other modules
/// this will handle targetting for now
fn ev_watch_castskill(
    mut commands: Commands,
    mut ev_castskill: EventReader<CastSkillEvent>,
    skill_q: Query<(Entity, &LabelName, Option<&Channel>, &Target), With<Skill>>,
    mut ev_sk2eval: EventWriter<EvalSkillEvent>,
    mut ev_channelingsk2eval: EventWriter<EvalChannelingSkillEvent>,
) {
    for ev in ev_castskill.iter() {
        for (skill_ent, skill_name, skill_channel, skill_target) in
            skill_q.iter().filter(|ent| ent.0 == ev.skill_ent.0)
        {
            info!(
                "CastSkillEvent {:?} {:?} ({:?}) => {:?}",
                skill_ent, skill_name.0, skill_target, ev.caster
            );
            if let Some(skill_channel) = skill_channel {
                commands
                    .entity(ev.caster)
                    .insert(Channel(skill_channel.0))
                    .insert(Casting {
                        skill_ent,
                        target_ent: ev.target,
                    });
            }
            match skill_channel {
                Some(skill_channel) => ev_channelingsk2eval.send(EvalChannelingSkillEvent {
                    skill: ev.skill_ent.0,
                    channel: *skill_channel,
                    target: ev.target,
                    caster: ev.caster,
                }),
                None => ev_sk2eval.send(EvalSkillEvent {
                    skill: ev.skill_ent.0,
                    target: ev.target,
                    caster: ev.caster,
                }),
            }
        }
        commands.insert_resource(NextState(WhoseTurn::System));
        // assign channel component to unit entities
    }
}

pub struct EvalSkillEvent {
    skill: Entity,
    target: Entity,
    caster: Entity,
}
pub struct EvalChannelingSkillEvent {
    skill: Entity,
    channel: Channel, // see if we need this
    target: Entity,
    caster: Entity,
}

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
                commands.insert_resource(NextState(SkillWheelStatus::Open));
            }
            NextInTurn::Enemy => {
                commands.insert_resource(NextState(NextInTurn::Player));
                commands.insert_resource(NextState(WhoseTurn::Enemy));
                commands.insert_resource(NextState(SkillWheelStatus::Closed));
                commands.insert_resource(NextState(TargetPromptStatus::Closed));
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
