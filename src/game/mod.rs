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
                .with_system(cast_skill_event.run_if(same_skill_selected))
                .into(),
        )
        .add_system_set(
            // also listen for cast skill event in enemy turn
            ConditionSet::new()
            .run_in_state(WhoseTurn::Enemy)
            .with_system(cast_skill_event)
            .into()
        )
        ;
    }
}

fn cast_skill_event(
    mut ev_castskill: EventReader<CastSkillEvent>,
    // only apply context window check when it's the player's turn
    whose_turn: Res<CurrentState<WhoseTurn>>,
    skill_q: Query<(Entity, &LabelName), With<Skill>>,
    mut commands: Commands,
) {
    for ev in ev_castskill.iter() {
        for (skill_ent, skill_name) in skill_q.iter() {
            if skill_ent == ev.skill_ent.0 {
                match whose_turn.0 {
                    WhoseTurn::Player => {
                        info!("CastSkillEvent {:?}", skill_name.name);
                        commands.insert_resource(NextState(SkillContextStatus::Closed));
                        // no longer your turn, eval the skill now
                        commands.insert_resource(NextState(WhoseTurn::System));
                    }
                    WhoseTurn::Enemy => {
                        info!("CastSkillEvent {:?}", skill_name.name);
                        // no longer your turn, eval the skill now
                        commands.insert_resource(NextState(WhoseTurn::System));
                    }
                    WhoseTurn::System => {}
                }
            }
        }
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