use bevy::prelude::*;

// TODO: better imports
// TODO: crate:Name too lengthy
use crate::Block;
use crate::Damage;
use crate::Heal;
use crate::Skill;
#[derive(Component)]
pub struct Learned(bool);

pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(
            SystemSet::new()
                .with_system(spawn_skill_basic_attack)
                .with_system(spawn_skill_basic_block)
                .with_system(spawn_skill_basic_heal),
        );
    }
}

fn spawn_skill_basic_attack(mut commands: Commands) {
    commands
        .spawn()
        .insert(Learned(true))
        .insert(Skill)
        .insert(crate::Name {
            name: "Attack".to_string(),
        })
        .insert(Damage { value: 7 });
}
fn spawn_skill_basic_block(mut commands: Commands) {
    commands
        .spawn()
        .insert(Learned(true))
        .insert(Skill)
        .insert(crate::Name {
            name: "Block".to_string(),
        })
        .insert(Block { value: 5 });
}
fn spawn_skill_basic_heal(mut commands: Commands) {
    commands
        .spawn()
        .insert(Learned(true))
        .insert(Skill)
        .insert(crate::Name {
            name: "Bandaid".to_string(),
        })
        .insert(Heal { value: 5 });
}
