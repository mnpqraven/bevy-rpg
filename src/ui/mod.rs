mod style;
mod combat;
mod env;

use bevy::prelude::*;

use combat::CombatUIPlugin;
use env::EnvUIPlugin;

use crate::game::component::SkillEnt;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CombatUIPlugin)
        .add_plugin(EnvUIPlugin);
    }
}

/// CP, UI Tag
#[derive(Component)]
struct ContextWindow;
/// CP, UI Tag
#[derive(Component)]
struct PromptWindow;
/// Vector of 2, pass true to same_skill_selected if both are equal
#[derive(Component, Debug)]
struct ContextHistory(Option<SkillEnt>);
/// State indicating whether the skill wheel should be visible
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SkillWheelStatus {
    Open,
    Closed,
}
#[derive(Component, Debug)]
struct SelectingSkill(Option<Entity>);
/// Event { Entity }: entity id of the target (by skill/user)
struct TargetSelectEvent(Entity);