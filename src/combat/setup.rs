use bevy::prelude::*;
use iyes_loopless::prelude::*;
use crate::ecs::component::*;
use super::{process::TurnOrderList, ControlMutex};

/// Query units and returns TurnOrderList
pub fn setup_turn_order(unit_q: Query<(Entity, &Speed)>, mut commands: Commands) {
    let mut query: Vec<(Entity, Speed)> = Vec::new();
    for (ent, speed_ptr) in unit_q.iter() {
        query.push((ent, *speed_ptr));
    }
    // NOTE: setup turn order here, refactor later
    let tol = TurnOrderList::new_sorted(query);
    commands.insert_resource(tol);
    // assigns the correct mutex
    commands.insert_resource(NextState(ControlMutex::Unit));
}

/// returns a vec of unit entities that can be chosen for a given Target type
/// if you're having troubles with borrow checking the query try using .to_readonly()
pub fn setup_target_bucket(
    unit_q_ro: Query<
        (Entity, Option<&Player>, Option<&Ally>, Option<&Enemy>),
        Or<(With<Player>, With<Ally>, With<Enemy>)>,
    >,
    target_type: Target,
    caster_ent: Entity,
    is_enemy_casting: bool
) -> Vec<Entity> {
    unit_q_ro
        .iter()
        .filter(
            // TODO: refactor this monstrosity
            |(unit_ent, player_tag, ally_tag, enemy_tag)| match is_enemy_casting {
                true => match target_type {
                    Target::AllyAndSelf => enemy_tag.is_some(),
                    Target::AllyButSelf => enemy_tag.is_some() && caster_ent != *unit_ent,
                    Target::AllyAOE => enemy_tag.is_some(),
                    Target::EnemyAndSelf => enemy_tag.is_none() || caster_ent == *unit_ent,
                    Target::EnemyButSelf => enemy_tag.is_none(),
                    Target::EnemyAOE => enemy_tag.is_none(),
                    Target::OnlySelf => caster_ent == *unit_ent,
                    Target::Any => true,
                    Target::AnyButSelf => caster_ent != *unit_ent,
                    Target::All => true,
                },
                // player/ally caster
                false => match target_type {
                    Target::AllyAndSelf => player_tag.is_some() || ally_tag.is_some(),
                    Target::AllyButSelf => {
                        (player_tag.is_some() || ally_tag.is_some()) && caster_ent != *unit_ent
                    }
                    Target::AllyAOE => player_tag.is_some() || ally_tag.is_some(),
                    Target::EnemyAndSelf => enemy_tag.is_some() || caster_ent == *unit_ent,
                    Target::EnemyButSelf => enemy_tag.is_some(),
                    Target::EnemyAOE => enemy_tag.is_some(),
                    Target::OnlySelf => caster_ent == *unit_ent,
                    Target::Any => true,
                    Target::AnyButSelf => caster_ent != *unit_ent,
                    Target::All => true,
                },
            },
        )
        .map(|i| i.0)
        .collect()
}
