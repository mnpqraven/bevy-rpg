use bevy::prelude::*;

// TODO: better imports
// TODO: crate:Name too lengthy
use crate::game::component::*;
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
        .insert(LabelName {
            name: "Attack".to_string(),
        })
        .insert(Damage { value: 7 });
}
fn spawn_skill_basic_block(mut commands: Commands) {
    commands
        .spawn()
        .insert(Learned(true))
        .insert(Skill)
        .insert(LabelName {
            name: "Block".to_string(),
        })
        .insert(Block { value: 5 });
}
fn spawn_skill_basic_heal(mut commands: Commands) {
    commands
        .spawn()
        .insert(Learned(true))
        .insert(Skill)
        .insert(LabelName {
            name: "Bandaid".to_string(),
        })
        .insert(Heal { value: 5 });
}
