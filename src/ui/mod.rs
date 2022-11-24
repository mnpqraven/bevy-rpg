mod combat;
mod env;
pub mod style;

use bevy::prelude::*;

use combat::CombatUIPlugin;
use env::EnvUIPlugin;

use crate::ecs::component::SkillMeta;
use self::style::load_fonts;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CombatUIPlugin)
            .add_plugin(EnvUIPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_fonts);
    }
}

/// CP, UI Tag
#[derive(Component)]
struct ContextWindow;
/// CP, UI Tag
#[derive(Component)]
struct PromptWindow;
/// Vector of 2, pass true to same_skill_selected if both are equal
#[derive(Resource, Debug)]
struct ContextHistory(Option<SkillMeta>);
#[derive(Resource, Debug)]
struct SelectingSkill(Option<Entity>);
#[derive(Resource, Debug)]
pub struct CurrentCaster(pub Option<Entity>);
/// Event { Entity }: entity id of the target (by skill/user)
struct TargetSelectEvent(Entity);

#[derive(Resource, Clone)]
pub struct FontSheet(pub Handle<Font>);
