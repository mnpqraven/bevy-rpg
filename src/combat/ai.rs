use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::{thread_rng, Rng};

use crate::{ecs::component::*, ui::CurrentCaster};

use super::{process::{TurnOrderList, gen_target_bucket}, ChooseAISkillEvent};
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(choose_skill.run_in_state(GameState::InCombat));
    }
}

fn choose_skill(
    skill_q: Query<(Entity, &SkillGroup, &Target), With<Skill>>,
    unit_q: Query<
        (Entity, Option<&Player>,Option<&Ally>, Option<&Enemy>),
        Or<(With<Player>, With<Ally>, With<Enemy>)>,
    >,
    turnorder: Res<TurnOrderList<Entity, Speed>>,
    mut ev_castskill: EventWriter<CastSkillEvent>,
    mut ev_choose_ai_skill: EventReader<ChooseAISkillEvent>,
    mut commands: Commands,
    current_caster: Res<CurrentCaster>,
) {
    for _ in ev_choose_ai_skill.iter() {
        if !&turnorder.is_empty() {
            let (unit_ent, _, ally_tag, enemy_tag) = unit_q
                .get(*turnorder.get_current().expect("turn order vec is blank"))
                .unwrap();
            let filter: SkillGroup = match true {
                true if ally_tag.is_some() => SkillGroup::Ally,
                true if enemy_tag.is_some() => SkillGroup::Enemy,
                _ => SkillGroup::Universal,
            };
            // gather all skills from either enemy grp or ally
            let pool: Vec<(Entity, &SkillGroup, &Target)> =
                skill_q.iter().filter(|item| item.1.eq(&filter)).collect();
            let rng_index = thread_rng().gen_range(0..pool.len());
            let target_type = pool[rng_index].2.clone();

            let filtered_units = gen_target_bucket(unit_q.to_readonly(), target_type, current_caster.0);

            commands.insert_resource(CurrentCaster(Some(unit_ent)));
            ev_castskill.send(CastSkillEvent {
                skill_ent: SkillEnt(pool[rng_index].0),
                target: filtered_units[0],
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
