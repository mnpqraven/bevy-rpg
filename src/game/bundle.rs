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
pub struct UnitBundle {
    pub name: LabelName,
    pub current_health: Health,
    pub max_health: MaxHealth,
    pub current_mana: Mana,
    pub max_mana: MaxMana,
    pub default_block: Block,
    pub speed: Speed
}
#[allow(dead_code)]
impl UnitBundle {
    /// creates a new unit from given hp/mp/block values
    pub fn new(name: LabelName, health: Health, mana: Mana, default_block: Block, speed: Speed) -> Self {
        Self {
            name,
            current_health: health.clone(),
            max_health: MaxHealth(health.0),
            current_mana: mana.clone(),
            max_mana: MaxMana(mana.0),
            default_block,
            speed
        }
    }
    /// creates a new unit with custom hp/mp
    pub fn new_injured(
        name: LabelName,
        current_health: Health,
        max_health: MaxHealth,
        current_mana: Mana,
        max_mana: MaxMana,
        default_block: Block,
        speed: Speed
    ) -> Self {
        Self {
            name,
            current_health,
            max_health,
            current_mana,
            max_mana,
            default_block,
            speed
        }
    }
}
