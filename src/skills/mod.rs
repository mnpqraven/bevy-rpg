use bevy::prelude::*;

use crate::game::component::*;
use crate::game::bundle::*;
pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(
            SystemSet::new()
                .with_system(spawn_skill_basic_attack)
                .with_system(spawn_skill_basic_block)
                .with_system(spawn_skill_basic_heal)
                .with_system(spawn_skill_bite)
                .with_system(spawn_skill_bash),
        );
    }
}

// TODO: serde + SoA
fn spawn_skill_basic_attack(mut commands: Commands) {
    commands
        .spawn_bundle(SkillBundle {
            skill_group: SkillGroup::Universal,
            name: LabelName {
                name: "Basic Strike".to_string(),
            },
            target: Target::Enemy,
            ..default()
        })
        .insert(Damage { value: 7 })
        .insert(Learned(true));
}
fn spawn_skill_basic_block(mut commands: Commands) {
    commands
        .spawn_bundle(SkillBundle {
            skill_group: SkillGroup::Universal,
            name: LabelName {
                name: "Basic Block".to_string(),
            },
            target: Target::Player,
            ..default()
        })
        .insert(Block { value: 6 })
        .insert(Learned(true));
}
fn spawn_skill_basic_heal(mut commands: Commands) {
    commands
        .spawn_bundle(SkillBundle {
            skill_group: SkillGroup::Universal,
            name: LabelName {
                name: "Basic Heal".to_string(),
            },
            target: Target::Ally,
            ..default()
        })
        .insert(Heal { value: 5 })
        .insert(Learned(true));
}
fn spawn_skill_bash(mut commands: Commands) {
    commands
        .spawn_bundle(SkillBundle {
            skill_group: SkillGroup::Ally,
            name: LabelName {
                name: "Bash".to_string(),
            },
            target: Target::Enemy,
            ..default()
        })
        .insert(Damage {value: 12})
        .insert(Mana { value: 25 })
        .insert(Learned(true));
}
fn spawn_skill_bite(mut commands: Commands) {
    commands
        .spawn_bundle(SkillBundle {
            skill_group: SkillGroup::Enemy,
            name: LabelName {
                name: "Bite".to_string(),
            },
            target: Target::Ally,
            ..default()
        })
        .insert(Damage { value: 13});
}

impl Default for SkillBundle {
    fn default() -> Self {
        Self {
            skill_group: SkillGroup::Universal,
            name: LabelName {
                name: String::from("Unnamed skill"),
            },
            skill: Skill,
            target: Target::Any
        }
    }
}
