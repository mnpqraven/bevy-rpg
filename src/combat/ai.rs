use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::{thread_rng, Rng};

use crate::{ecs::component::*, ui::CurrentCaster};

use super::{process::TurnOrderList, ChooseAISkillEvent};
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(choose_skill.run_in_state(GameState::InCombat));
    }
}

fn choose_skill(
    skill_q: Query<(Entity, &SkillGroup, &Target), With<Skill>>,
    unit_type: Query<
        (Entity, Option<&Ally>, Option<&Enemy>, &Speed),
        Or<(With<Player>, With<Ally>, With<Enemy>)>,
    >,
    target_search_q: Query<
        (Entity, Option<&Player>, Option<&Ally>, Option<&Enemy>),
        Or<(With<Player>, With<Ally>, With<Enemy>)>,
    >,
    turnorder: Res<TurnOrderList<Entity, Speed>>,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    mut ev_choose_ai_skill: EventReader<ChooseAISkillEvent>,
    mut commands: Commands,
) {
    for _ in ev_choose_ai_skill.iter() {
        if !&turnorder.is_empty() {
            let (unit_ent, ally_tag, enemy_tag, _) = unit_type
                .get(*turnorder.get_current().expect("turn order vec is blank"))
                .unwrap();
            let filter: SkillGroup = match true {
                true if ally_tag.is_some() => SkillGroup::Ally,
                true if enemy_tag.is_some() => SkillGroup::Enemy,
                _ => SkillGroup::Universal,
            };
            let pool: Vec<(Entity, &SkillGroup, &Target)> =
                skill_q.iter().filter(|item| item.1.eq(&filter)).collect();
            let rng_index = thread_rng().gen_range(0..pool.len());
            // hard code target selecting
            let target_list: Vec<(Entity, Option<&Player>, Option<&Ally>, Option<&Enemy>)> =
                // TODO: refactor this monstrosity
                match pool[rng_index].2 {
                    Target::Player | Target::NoneButSelf => target_search_q
                        .iter()
                        .filter(|item| item.1.is_some())
                        .collect(),
                    Target::AllyAndSelf | Target::AllyAOE => target_search_q
                        .iter()
                        .filter(|item| item.2.is_some() || item.1.is_some())
                        .collect(),
                    Target::AllyButSelf => target_search_q
                        .iter()
                        .filter(|item| item.2.is_some() && item.1.is_none())
                        .collect(),
                    Target::Enemy | Target::EnemyAOE => target_search_q
                        .iter()
                        .filter(|item| item.3.is_some())
                        .collect(),
                    Target::Any => target_search_q
                        .iter()
                        .filter(|item| item.1.is_some() || item.2.is_some() || item.3.is_some())
                        .collect(),
                    Target::AnyButSelf => target_search_q
                        .iter()
                        .filter(|item| item.1.is_none())
                        .collect(),
                };
            let target_ent: Entity = target_list[0].0;
            commands.insert_resource(CurrentCaster(Some(unit_ent)));
            ev_castskill.send(CastSkillEvent {
                skill_ent: SkillEnt(pool[rng_index].0),
                target: target_ent,
                caster: unit_ent,
            });
        }
    }
}

#[cfg(test)]
mod tests {

    use rand::thread_rng;
    use rand::Rng;

    #[test]
    #[ignore = "irrelevant"]
    fn rando() {
        let mut rng = thread_rng();
        let r: i32 = rng.gen_range(0..=1);
        for _ in 1..1000 {
            println!("{}", r);
        }
    }
}
