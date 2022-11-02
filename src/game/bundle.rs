use bevy::prelude::*;

use crate::ecs::component::*;

/// only required fields for a skill, you must manually add other optional fields
#[derive(Bundle)]
pub struct SkillBundle {
    pub name: LabelName,
    pub skill_group: SkillGroup,
    pub target: Target,
    pub skill: Skill, // tag
}
#[derive(Bundle)]
pub struct CharacterBundle {
    pub name: LabelName,
    pub current_health: Health,
    pub current_block: Block,
}