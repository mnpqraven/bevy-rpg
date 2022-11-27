mod parser;

use bevy::prelude::*;

use crate::ecs::component::*;
use crate::game::bundle::*;

use self::parser::scan_skillbook_yaml;
use self::parser::SkillEntry;
pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set(SystemSet::new().with_system(load_skillbook));
    }
}

/// Read data from a Vec<SkillDataTable> resource and then spawn skills
fn load_skillbook(mut commands: Commands) {
    let skilldata: Vec<SkillEntry> = scan_skillbook_yaml();
    for skill in skilldata.iter() {
        let skill_ent = commands
            .spawn(SkillBundle {
                name: LabelName(skill.label_name.to_owned()),
                skill_group: SkillGroupList(skill.skill_group.clone()),
                target: skill.target.clone(),
                ..default()
            })
            .id();
        // TODO: refactor this monstrosity
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
        commands.entity(skill_ent).insert((
            skill.learned,
            LearnableArchetypes(skill.learnable_archetypes.to_owned()),
        ));
    }
}

impl Default for SkillBundle {
    fn default() -> Self {
        Self {
            skill_group: SkillGroupList(vec![SkillGroup::Universal]),
            name: LabelName(String::from("Unnamed skill")),
            skill: Skill,
            target: Target::Any,
        }
    }
}
