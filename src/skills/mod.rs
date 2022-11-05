mod parser;

use bevy::prelude::*;

use crate::game::bundle::*;
use crate::ecs::component::*;

use self::parser::scan_skillbook;
use self::parser::SkillDataTable;
pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(SystemSet::new().with_system(load_skillbook));
    }
}

/// Read data from a Vec<SkillDataTable> resource and then spawn skills
fn load_skillbook(mut commands: Commands) {
    let skilldata: Vec<SkillDataTable> = scan_skillbook();
    for skill in skilldata.iter() {
        let skill_ent = commands
            .spawn_bundle(SkillBundle {
                name: LabelName(skill.label_name.to_owned()),
                skill_group: skill.skill_group.clone(),
                target: skill.target.clone(),
                ..default()
            })
            .id();
        if skill.damage.is_some() {
            commands
                .entity(skill_ent)
                .insert(Damage(skill.damage.unwrap()));
        }
        if skill.block.is_some() {
            commands
                .entity(skill_ent)
                .insert(Block(skill.block.unwrap()));
        }
        if skill.heal.is_some() {
            commands.entity(skill_ent).insert(Heal(skill.heal.unwrap()));
        }
        if skill.channel.is_some() {
            commands
                .entity(skill_ent)
                .insert(Channel(skill.channel.unwrap()));
        }
        if skill.learned.is_some() {
            commands
                .entity(skill_ent)
                .insert(Learned(skill.learned.unwrap()));
        }
    }
}

impl Default for SkillBundle {
    fn default() -> Self {
        Self {
            skill_group: SkillGroup::Universal,
            name: LabelName(String::from("Unnamed skill")),
            skill: Skill,
            target: Target::Any,
        }
    }
}
