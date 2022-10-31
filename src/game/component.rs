//! ===========================================================================
//! Global components that are being used by different modules
//! ===========================================================================

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

/// CP
/// User-controlled component
#[derive(Component, Debug)]
pub struct Player;
/// CP
/// User's allies
#[derive(Component, Debug)]
pub struct Ally;
/// CP
#[derive(Component, Debug)]
pub struct Enemy;
/// CP
/// LabelName (Crate Name) to avoid conflict with bevy's Name struct
#[derive(Component, Clone, Debug, Inspectable)]
pub struct LabelName {
    pub name: String,
}
/// CP
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
}
/// CP
/// whether a skill can only be cast by frienlies or enemies, or both
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
/// carries skill entity id
#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SkillEnt(pub Entity);

// STATS ============
/// CP
#[derive(Component, Clone)]
pub struct Health {
    pub value: i32,
}
/// CP
#[derive(Component, Clone)]
pub struct MaxHealth {
    pub value: i32,
}
/// CP
#[derive(Component)]
pub struct Mana {
    pub value: i32,
}
/// CP
#[derive(Component)]
pub struct Damage {
    pub value: i32,
}
/// CP
#[derive(Component, Clone)]
pub struct Block {
    pub value: i32,
}
impl Default for Block {
    fn default() -> Self {
        Self { value: 0 }
    }
}
/// CP
#[derive(Component)]
pub struct Heal {
    pub value: i32,
}
/// CP
/// whether the character is moving in env state
#[derive(Component)]
pub struct IsMoving(pub bool);

#[derive(Component, Debug)]
pub struct TargetEnt(pub Entity);

// UI ===============
#[derive(Component)]
pub struct SkillIcon;

/// Event { SkillEnt }
pub struct CastSkillEvent {
    pub skill_ent: SkillEnt,
    pub target: Entity,
}

/// State indicating whether it's the character's turn yet and can act
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhoseTurn {
    Player,
    Enemy,
    System,
}

/// State indicating whether the character is interacting with the open world or in combat
/// OutOfCombat: when character is in world, can move
/// InCombat: when character is in combat, can't move
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    InCombat,
    OutOfCombat,
}
