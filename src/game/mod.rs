use crate::{
    game::component::*,
    menu::SkillContextStatus, combat::WhoseTurn,
};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub mod component;
pub mod sprites;
pub mod bundle;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(sprites::SpritePlugin)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(SkillContextStatus::Open)
                // only cast after you see the
                // skill's details
                .into(),
        )
        .add_system_set(
            // also listen for cast skill event in enemy turn
            ConditionSet::new()
            .run_in_state(WhoseTurn::Enemy)
            .into()
        )
        ;
    }
}


/// returns whether if the skill the user click is the same as the context
/// window skill spawned on the screen
fn same_skill_selected(history: Res<ContextHistory>) -> bool {
    match history.0.get(1).is_some() {
        true =>  history.0[0].0 == history.0[1].0,
        false => false
    }
}