mod parser;

use bevy::prelude::*;

use crate::game::component::*;
use crate::game::bundle::*;

use self::parser::SkillDataTable;
use self::parser::scan_skillbook;
pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(
            SystemSet::new()
                // .with_system(spawn_skill_basic_attack)
                // .with_system(spawn_skill_basic_block)
                // .with_system(spawn_skill_basic_heal)
                // .with_system(spawn_skill_bite)
                // .with_system(spawn_skill_bash)
                .with_system(test_new_load)
                ,
        );
    }
}

// do we need to convert to SoA ?
fn test_new_load(
    mut commands: Commands
) {
    let skilldata: Vec<SkillDataTable> = scan_skillbook();
    for skill in skilldata.iter() {
        let skill_ent = commands.
        spawn_bundle(SkillBundle {
            name: LabelName { name: skill.label_name.to_owned() },
            skill_group: skill.skill_group.clone(),
            target: skill.target.clone(),
            ..default()
        })
        .id();
        if skill.damage.is_some() {
            commands.entity(skill_ent)
            .insert(Damage {value: skill.damage.unwrap() });
        }
        if skill.block.is_some() {
            commands.entity(skill_ent)
            .insert(Damage {value: skill.block.unwrap() });
        }
        if skill.heal.is_some() {
            commands.entity(skill_ent)
            .insert(Damage {value: skill.heal.unwrap() });
        }
        if skill.learned.is_some() {
            commands.entity(skill_ent)
            .insert(Learned (skill.learned.unwrap()));
        }
    }
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
