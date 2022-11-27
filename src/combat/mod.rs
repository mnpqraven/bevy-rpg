mod ai;
mod eval;
pub mod process;
pub mod setup;
mod tests;
use crate::combat::eval::{eval_block, eval_damage, eval_heal};
use crate::ecs::component::*;
use crate::game::despawn_with;
use crate::game::sprites::{spawn_combat_allysp, spawn_combat_enemysp};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use self::ai::AiPlugin;
use self::process::TurnOrderList;

pub struct CombatPlugin;
/// what's having control of the game ?
/// unit: either a player, ally or enemy is occupying the game
/// System: state for eval, playing animation ..
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControlMutex {
    // for adding loopless state, we don't want enter systems running at the start
    // Can put setup function here
    Startup,
    Unit, // player, ally, enemy go here
    SystemTurn,
    SystemReward,
}

/// hp trigger, story event, etc
pub struct _SpecialTriggerEvent;
pub struct _GameOverEvent;

pub struct _FightClearedEvent;
pub struct UnitKilledEvent(Entity);

pub struct TurnEndEvent;

/// How long the animation should be running
/// We are using a global timer so we don't
/// reset the timer on every system call
#[derive(Resource)]
struct AnimationLengthConfig {
    // TODO: this is supposed to be updated for different animations
    timer: Timer,
}
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AiPlugin)
            .add_loopless_state(ControlMutex::Startup)
            .insert_resource(TurnOrderList::<Entity, Speed>::new())
            .insert_resource(AnimationLengthConfig {
                timer: Timer::from_seconds(2., TimerMode::Once),
            })
            .insert_resource(process::TurnOrderList::<Entity, Speed>::new())
            // States
            .add_enter_system_set(
                ControlMutex::Unit,
                SystemSet::new()
                    .label("eval_turn_start")
                    .with_system(eval_turn_start),
            )
            .add_enter_system_set(
                ControlMutex::Unit,
                SystemSet::new()
                    .label("setup_resource")
                    .before("eval_turn_start")
                    // .with_system(setup_resource),
            )
            .add_exit_system(ControlMutex::Unit, despawn_with::<SkillIcon>)
            .add_system_set(
                // ControlMutex::SystemTurn,
                SystemSet::new()
                    .with_system(eval::eval_instant_skill)
                    .with_system(eval::eval_channeling_skill),
            )
            .add_enter_system_set(
                ControlMutex::SystemReward,
                ConditionSet::new().with_system(show_reward).into(),
            )
            .add_system(animate_skill.run_in_state(ControlMutex::SystemTurn))
            // TODO: fix hack
            .add_exit_system_set(
                GameState::OutOfCombat,
                ConditionSet::new()
                    // TODO: move back to sprite module, resolve data race
                    .with_system(spawn_combat_allysp)
                    .with_system(spawn_combat_enemysp)
                    .into(),
            )
            .add_enter_system(GameState::InCombat, setup::setup_turn_order)
            // Events
            .add_event::<EnterWhiteOutEvent>()
            .add_event::<ChooseAISkillEvent>()
            .add_event::<UnitKilledEvent>()
            .add_event::<CastSkillEvent>()
            .add_event::<EvalSkillEvent>()
            .add_event::<EvalChannelingSkillEvent>()
            .add_event::<TurnEndEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_system(ev_enemykilled)
                    .with_system(evread_castskill)
                    .with_system(evread_endturn),
            );
    }
}

// ----------------------------------------------------------------------------
// TODO: move code chunk + finish
/// animate the skill animation after casting a skill
fn animate_skill(
    time: Res<Time>,
    mut config: ResMut<AnimationLengthConfig>,
    mut ev_endturn: EventWriter<TurnEndEvent>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut texture_q: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<CombatSprite>,
    >,
    turn_order: Res<TurnOrderList<Entity, Speed>>,
) {
    if !texture_q.is_empty() {
        let (mut animation_timer, mut sprite, handle) = texture_q
            .get_mut(*turn_order.get_current().expect("TurnOrderList should have data by now"))
            .expect("no texture with combatsprite found");
        config.timer.tick(time.delta());
        if !config.timer.just_finished() {
            // animation phase
            // TODO: refactor to sprite module
            animation_timer.tick(time.delta());
            if animation_timer.just_finished() {
                let texture_atlas = texture_atlases.get(handle).unwrap();
                // next index in sprite sheet
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                animation_timer.reset();
            }
            // TODO: go back to first sprite index
        } else {
            // sending event, prep for exiting ControlMutex::System
            sprite.index = 0;
            ev_endturn.send(TurnEndEvent);
            config.timer.reset();
        }
    }
}
// ----------------------------------------------------------------------------
fn eval_turn_start(
    // player: Query<Entity, With<Player>>,
    mut casting_ally_q: Query<(Entity, &mut Channel, &Casting), With<Player>>,
    skill_q: Query<(Option<&Damage>, Option<&Heal>, Option<&Block>), With<Skill>>,
    mut unit_q: Query<(&mut Health, &mut Block), Without<Skill>>,
    mut ev_endturn: EventWriter<TurnEndEvent>,
    mut commands: Commands,
    turn_order: ResMut<TurnOrderList<Entity, Speed>>,
    unit_tag_q: Query<
        (Option<&Player>, Option<&Ally>, Option<&Enemy>),
        Or<(With<Player>, With<Ally>, With<Enemy>)>,
    >,
    mut ev_choose_ai_skill: EventWriter<ChooseAISkillEvent>,
) {
    info!("[ENTER] ControlMutex::Unit: eval_turn_start");
    info!("TurnStart for {:?}", turn_order.get_current());
    info!("TurnOrderList debug {:?}", turn_order);

    let (_, _, unit_enemy_tag) = unit_tag_q
        .get(*turn_order.get_current().expect("turn order vec is blank"))
        .expect("should have at least 1 unit result");

    if unit_enemy_tag.is_some() {
        ev_choose_ai_skill.send(ChooseAISkillEvent);
        // BUG: sending TurnEndEvent here messes up ordering
        // TODO: test to make sure this omit is correct
        // ev_endturn.send(TurnEndEvent);
    } else {
        // opens skill wheel (hacky)
        commands.insert_resource(NextState(SkillWheelStatus::Open));
        for (unit_ent, mut unit_channel, unit_casting) in &mut casting_ally_q {
            if unit_channel.0 == 1 {
                let (skill_damage, skill_heal, skill_block) = skill_q
                    .get(unit_casting.skill_ent)
                    .expect("can't get skill from entity id");
                let (mut target_health, mut target_block) = unit_q
                    .get_mut(unit_casting.target_ent)
                    .expect("can't get target from entity id");
                // TODO: test
                eval_block(skill_block, &mut target_block);
                eval_heal(skill_heal, &mut target_health);
                eval_damage(skill_damage, &mut target_health, &mut target_block);

                commands.entity(unit_ent).remove::<Channel>();
                commands.entity(unit_ent).remove::<Casting>();
                // no longer casting, not ending turn + allow choosing skill
            } else {
                // skips player turn when casting
                ev_endturn.send(TurnEndEvent);
                unit_channel.0 -= 1;
            }
        }
    }
}
pub struct ChooseAISkillEvent;

/// Listens to SkillCastEvent from other modules
///
/// This will handle targetting for now
fn evread_castskill(
    mut commands: Commands,
    mut ev_castskill: EventReader<CastSkillEvent>,
    skill_q: Query<(Entity, &LabelName, Option<&Channel>, &Target), With<Skill>>,
    mut ev_sk2eval: EventWriter<EvalSkillEvent>,
    mut ev_channelingsk2eval: EventWriter<EvalChannelingSkillEvent>,
    mut ev_endturn: EventWriter<TurnEndEvent>,
) {
    for ev in ev_castskill.iter() {
        for (skill_ent, skill_name, skill_channel, skill_target) in
            skill_q.iter().filter(|ent| ent.0 == ev.skill_ent.0)
        {
            info!(
                "CastSkillEvent {:?} {:?} ({:?}) caster: {:?} => target: {:?}",
                skill_ent, skill_name.0, skill_target, ev.caster, ev.target
            );
            if let Some(skill_channel) = skill_channel {
                commands.entity(ev.caster).insert((
                    Channel(skill_channel.0),
                    Casting {
                        skill_ent,
                        target_ent: ev.target,
                    },
                ));
                ev_channelingsk2eval.send(EvalChannelingSkillEvent {
                    skill: ev.skill_ent.0,
                    channel: *skill_channel,
                    target: ev.target,
                    caster: ev.caster,
                });
            } else {
                ev_sk2eval.send(EvalSkillEvent {
                    skill: ev.skill_ent.0,
                    target: ev.target,
                    caster: ev.caster,
                });
            }
        }
        ev_endturn.send(TurnEndEvent);
        // assign channel component to unit entities
    }
}

/// Evaluation event after a skill is cast
/// * skill: Entity
/// * target: Entity
/// * caster: Entity
#[allow(dead_code)]
pub struct EvalSkillEvent {
    skill: Entity,
    target: Entity,
    caster: Entity,
}

pub struct UIBarChangeEvent {
    pub master_unit: Entity,
}
/// Evaluation event after a channeling skill is cast
/// * skill: Entity
/// * channel: Channel
/// * target: Entity
/// * caster: Entity
#[allow(dead_code)]
pub struct EvalChannelingSkillEvent {
    skill: Entity,
    channel: Channel, // see if we need this
    target: Entity,
    caster: Entity,
}

/// enters White Out when taking lethal damage
/// player is at negative health but doesn't die yet,
/// and will die if they get attacked again (opens up pre-casting heal)
/// leaves White Out when they're at positive health
#[derive(Component)]
pub struct WhiteOut;
pub struct EnterWhiteOutEvent(Entity);

/// Listens to TurnEndEvent
fn evread_endturn(
    mut commands: Commands,
    mut ev_endturn: EventReader<TurnEndEvent>,
    control_mutex: Res<CurrentState<ControlMutex>>,
    mut turn_order: ResMut<TurnOrderList<Entity, Speed>>,
) {
    for _ in ev_endturn.iter() {
        info!("TurnEndEvent");
        match control_mutex.0 {
            ControlMutex::Unit => commands.insert_resource(NextState(ControlMutex::SystemTurn)),
            ControlMutex::Startup => commands.insert_resource(NextState(ControlMutex::Unit)),
            ControlMutex::SystemTurn => {
                turn_order.next().expect("should not be empty");
                commands.insert_resource(NextState(ControlMutex::Unit));
            }
            ControlMutex::SystemReward => {} // TurnEndEvent should never trigger in SystemReward
        }
    }
}
/// Listens to EnemyKilledEvent
fn ev_enemykilled(mut ev_enemykilled: EventReader<UnitKilledEvent>, mut commands: Commands) {
    for ev in ev_enemykilled.iter() {
        info!("{:?} slain", ev.0);
        commands.entity(ev.0).despawn();
        info!("moving to ControlMutex::SystemReward");
        commands.insert_resource(NextState(ControlMutex::SystemReward))
    }
}

fn show_reward() {
    info!("you won smth");
}
