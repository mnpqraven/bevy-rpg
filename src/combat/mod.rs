mod ai;
mod eval;
mod process;
use crate::combat::eval::{eval_block, eval_damage, eval_heal};
use crate::ecs::component::*;
use crate::game::despawn_with;
use crate::ui::CurrentCaster;
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
pub struct _SpecialTriggerEvent;
pub struct _GameOverEvent;

pub struct _FightClearedEvent;
pub struct EnemyKilledEvent(Entity);

pub struct TurnStartEvent;
pub struct EnemyTurnStartEvent;

pub struct TurnEndEvent;

/// How long the animation should be running
/// We are using a global timer so we don't
/// reset the timer on every system call
struct AnimationLengthConfig {
    // TODO: this is supposed to be updated for different animations
    timer: Timer,
}
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
            .add_system(evread_castskill)
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
                    .with_system(eval::eval_instant_skill)
                    .with_system(eval::eval_channeling_skill)
                    .into(),
            )
            .insert_resource(AnimationLengthConfig {
                timer: Timer::from_seconds(2., false),
            })
            .add_system(animate_skill.run_in_state(WhoseTurn::System))
            .insert_resource(process::TurnOrderList::<Entity, Speed>::new())
            // TODO: fix hack
            .add_exit_system_set(
                GameState::OutOfCombat,
                ConditionSet::new()
                    .label("setup_spawn")
                    .before("setup_turn_order")
                    .with_system(spawn_combat_units)
                    .into(),
            )
            .add_enter_system_set(
                GameState::InCombat,
                ConditionSet::new()
                    .label("setup_turn_order")
                    .after("setup_spawn")
                    .with_system(process::generate_turn_order)
                    .into(),
            );
    }
}

// TODO: bind this with sprite
fn spawn_combat_units(mut commands: Commands) {
    // TODO: spawn entities with required metadata, tags
    // can leave sprite out for now
    info!("spawning entities.. ");
    commands
        .spawn()
        .insert(LabelName("Othi dummy (speed dev)".to_string()))
        .insert(Speed(0));
    commands
        .spawn()
        .insert(LabelName("Ally dummy (speed dev)".to_string()))
        .insert(Speed(1));
    commands
        .spawn()
        .insert(LabelName("enemy dummy (speed dev)".to_string()))
        .insert(Speed(-1));
    info!("spawned");
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
    caster: Res<CurrentCaster>,
) {
    config.timer.tick(time.delta());
    if !config.timer.just_finished() {
        // animation phase
        // TODO: refactor to sprite module
        let (mut animation_timer, mut sprite, handle) =
            texture_q.get_mut(caster.0.unwrap()).unwrap();
        animation_timer.tick(time.delta());
        if animation_timer.just_finished() {
            let texture_atlas = texture_atlases.get(handle).unwrap();
            // next index in sprite sheet
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            animation_timer.reset();
            // debug!("step {:?}", caster);
        }
        // TODO: go back to first sprite index
    } else {
        // sending event, prep for exiting WhoseTurn::System
        ev_endturn.send(TurnEndEvent);
        config.timer.reset();
    }
}
// ----------------------------------------------------------------------------

// TODO: modularize
/// Prepper when the player's turn starts
fn ev_player_turn_start(
    mut commands: Commands,
    mut casting_ally_q: Query<(Entity, &mut Channel, &Casting), With<Player>>,
    skill_q: Query<(Option<&Damage>, Option<&Heal>, Option<&Block>), With<Skill>>,
    mut unit_q: Query<(&mut Health, &mut Block), Without<Skill>>,
    mut ev_endturn: EventWriter<TurnEndEvent>,
) {
    info!("WhoseTurn::Player");
    // handle channel
    for (unit_ent, mut unit_channel, unit_casting) in &mut casting_ally_q {
        if unit_channel.0 == 1 {
            debug!("{:?}", unit_casting.skill_ent);

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
            // allow choosing skill
        } else {
            // skips player turn when casting
            // TODO: skips to ally turn when implemented
            ev_endturn.send(TurnEndEvent);
            unit_channel.0 -= 1;
        }
    }
}
/// Prepper when the enemy's turn starts
/// FIXME: caster always default to the only existing enemy, will panic if there's
/// multiple enemies.
/// TODO: system to decide enemy ent
fn ev_enemy_turn_start(
    // TODO: refactor to other chunks later
    player: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    enemy_skill_q: Query<(Entity, &SkillGroup), With<Skill>>,
    mut commands: Commands,
) {
    info!("WhoseTurn::Enemy");
    // only enemy skills rn, expand to universal later when we restructure skill data
    // TODO: fetch skill ent from enemy ai algorithm
    for (enemy_skill_ent, _) in enemy_skill_q
        .iter()
        .filter(|(_, grp)| **grp == SkillGroup::Enemy)
    {
        let caster_ent = enemies.iter().next().unwrap();
        commands.insert_resource(CurrentCaster(Some(caster_ent)));
        ev_castskill.send(CastSkillEvent {
            skill_ent: SkillEnt(enemy_skill_ent),
            target: player.single(),
            caster: caster_ent,
        });
    }
}

/// Listens to SkillCastEvent from other modules
///
/// This will handle targetting for now
fn evread_castskill(
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
                "CastSkillEvent {:?} {:?} ({:?}) caster: {:?} => target: {:?}",
                skill_ent, skill_name.0, skill_target, ev.caster, ev.target
            );
            if let Some(skill_channel) = skill_channel {
                commands
                    .entity(ev.caster)
                    .insert(Channel(skill_channel.0))
                    .insert(Casting {
                        skill_ent,
                        target_ent: ev.target,
                    });
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
        commands.insert_resource(NextState(WhoseTurn::System));
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

// enters White Out when taking lethal damage
// player is at negative health but doesn't die yet,
// and will die if they get attacked again (opens up pre-casting heal)
// leaves White Out when they're at positive health
#[derive(Component)]
pub struct WhiteOut;
pub struct EnterWhiteOutEvent(Entity);

/// Listens to TurnEndEvent
fn evread_endturn(
    mut commands: Commands,
    mut ev_endturn: EventReader<TurnEndEvent>,
    next_in_turn: Res<CurrentState<NextInTurn>>,
) {
    // time implement prototype
    // TODO: needs to be in normal system and run every frame
    // but only send once
    // tick in event ?
    // config.timer.tick(time.delta());
    // if config.timer.finished() {}
    for _ in ev_endturn.iter() {
        info!("TurnEndEvent");
        // see if blocking with timer is works here
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
            }
        }
    }
}
/// Listens to EnemyKilledEvent
fn ev_enemykilled(mut ev_enemykilled: EventReader<EnemyKilledEvent>, mut commands: Commands) {
    for ev in ev_enemykilled.iter() {
        info!("{:?} slain", ev.0);
        commands.entity(ev.0).despawn();
    }
}
