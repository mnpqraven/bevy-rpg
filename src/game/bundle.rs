use bevy::prelude::*;

use crate::game::component::*;

#[derive(Bundle)]
pub struct SkillBundle {
    pub skill_group: SkillGroup,
    pub name: LabelName,
    pub skill: Skill, // tag
}
#[derive(Bundle)]
pub struct CharacterBundle {
    pub name: LabelName,
    pub current_health: Health,
    pub current_block: Block,
}