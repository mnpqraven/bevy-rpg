use bevy::prelude::*;

/// CP
/// User-controlled component
#[derive(Component, Debug)]
pub struct Player;
/// CP
#[derive(Component)]
pub struct Enemy;

/// CP
/// LabelName (Crate Name) to avoid conflict with bevy's Name struct
#[derive(Component, Clone, Debug)]
pub struct LabelName {
    pub name: String,
}

/// CP, tag
#[derive(Component, Debug)]
pub struct Skill;
/// CP
#[derive(Component)]
pub struct Learned(pub bool);
/// CP
#[derive(Component, Debug, Copy, Clone)]
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
/// CP, movement direction, should(?) be linked with keyboard input
#[derive(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
/// CP
/// whether the character is moving in env state
#[derive(Component)]
pub struct IsMoving(pub bool);

/// CP
/// denotes the target that the character's skill will have effect on
#[derive(Component)]
pub struct Target(pub Entity);

/// CP
/// whether a skill can only be cast by frienlies or enemies, or both
#[derive(Component, PartialEq, Eq)]
pub enum SkillGroup {
    Ally,
    Enemy,
    Universal,
}

// UI ===============
/// CP, Tag
#[derive(Component)]
pub struct ContextWindow;
#[derive(Component)]
pub struct SkillIcon;

/// Vector of 2, pass true to same_skill_selected if both are equal
#[derive(Component, Debug)]
pub struct ContextHistory(pub Vec<SkillEnt>);
/// Event { SkillEnt }
pub struct CastSkillEvent {
    pub skill_ent: SkillEnt,
    pub target: Entity
}
