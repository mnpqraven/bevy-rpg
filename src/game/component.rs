use bevy::prelude::*;

// COMPONENT ==================================================================
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

/// CP
#[derive(Component, Debug)]
pub struct Skill;
/// CP
#[derive(Component)]
pub struct Learned(pub bool);

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
// #[derive(Component)]
// pub struct Mana(i32);
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
#[derive(Component)]
pub struct IsMoving(pub bool);
