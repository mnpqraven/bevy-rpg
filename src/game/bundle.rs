use bevy::prelude::*;

use crate::ecs::component::*;

/// Skill bundle for default skill with minimum required parameters
/// you must manually add other optional fields
#[derive(Bundle)]
pub struct SkillBundle {
    pub name: LabelName,
    pub skill_group: SkillGroup,
    pub target: Target,
    pub skill: Skill, // tag
}
/// Character bundle for default stats
/// you must manually add other optional fields
#[derive(Bundle)]
pub struct CharacterBundle {
    pub name: LabelName,
    pub current_health: Health,
    pub max_health: MaxHealth,
    pub current_block: Block,
}