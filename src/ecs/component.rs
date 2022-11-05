//! ===========================================================================
//! Global components that are being used by different modules
//! TODO: frequent check + maintenance of visibility scope
//! ===========================================================================

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

/// CP
///
/// User-controlled component
#[derive(Component, Debug)]
pub struct Player;
/// CP
///
/// User's allies
#[derive(Component, Debug)]
pub struct Ally;
/// CP
#[derive(Component, Debug)]
pub struct Enemy;
/// CP
///
/// LabelName (Crate Name) to avoid conflict with bevy's Name struct
#[derive(Component, Clone, Debug, Inspectable)]
pub struct LabelName(pub String);
/// CP
///
/// denotes the targetting type that the character's skill can have effect on
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "target")]
pub enum Target {
    Player,
    Ally,
    AllyButSelf,
    AllyAOE,
    Enemy,
    EnemyAOE,
    Any,
    AnyButSelf,
    NoneButSelf,
}
/// CP
///
/// Whether a skill can only be cast by frienlies or enemies, or both
#[derive(Component, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename = "skill_group")]
pub enum SkillGroup {
    Ally,
    Enemy,
    Universal,
}
/// CP, tag
#[derive(Component, Debug)]
pub struct Skill;
/// CP
#[derive(Component)]
pub struct Learned(pub bool);
/// CP
///
/// carries skill entity id
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SkillEnt(pub Entity);

// STATS ============
/// CP
#[derive(Component, Clone)]
pub struct Health(pub i32);
/// CP
#[derive(Component, Clone)]
pub struct MaxHealth(pub i32);
/// CP
#[derive(Component)]
pub struct Mana(pub i32);
/// CP
#[derive(Component)]
pub struct MaxMana(pub i32);
/// CP
#[derive(Component)]
pub struct Damage(pub i32);
/// CP
#[derive(Component, Clone, Default)]
pub struct Block(pub i32);

/// CP
#[derive(Component, Debug)]
pub struct Heal(pub i32);
/// CP
///
/// whether the character is moving in env state
#[derive(Component)]
pub struct IsMoving(pub bool);

#[derive(Component, Debug)]
pub struct TargetEnt(pub Entity);

#[derive(Component, Debug, Inspectable, Clone, Copy)]
pub struct Channel(pub u32);
#[derive(Component)]
pub struct Casting {
    pub skill_ent: Entity,
    pub target_ent: Entity,
}

// UI ===============
#[derive(Component)]
pub struct SkillIcon;
/// State indicating whether the skill wheel should be visible
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SkillWheelStatus {
    Open,
    Closed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetPromptStatus {
    Open,
    Closed,
}

/// Event { SkillEnt, target, caster }
pub struct CastSkillEvent {
    pub skill_ent: SkillEnt,
    pub target: Entity,
    pub caster: Entity,
}

/// State indicating whether it's the character's turn yet and can act
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhoseTurn {
    Player,
    Enemy,
    System,
}

/// State indicating whether the character is interacting with the open world or in combat
///
/// OutOfCombat: when character is in world, can move
///
/// InCombat: when character is in combat, can't move
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    InCombat,
    OutOfCombat,
}
