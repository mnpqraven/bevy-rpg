use bevy::prelude::*;
use crate::game::component::*;

pub struct Math;
impl Plugin for Math {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
fn calc_block(
    mut player: Query<(&LabelName, &Health, &mut Block), With<Player>>,
    skills: Query<(&LabelName, &Block), (With<Skill>, Without<Player>)>,
    // Without for disjoined query
) {
    let (player_name, player_health, mut player_block) = player.single_mut();
    print_player_stat(&player_name, &player_block, &player_health);
    for (skill_name, block) in skills.iter() {
        println!(
            "{}'s using defensive skill {} ({})",
            player_name.name, skill_name.name, block.value
        );
        player_block.value += block.value;
    }
    print_player_stat(&player_name, &player_block, &player_health);
    println!("====================");
}
fn calc_damage(
    mut player: Query<(&LabelName, &mut Block, &mut Health), With<Player>>,
    skills: Query<(&LabelName, &Damage), (With<Skill>, Without<Player>)>,
) {
    let (player_name, mut player_block, mut player_health) = player.single_mut();
    print_player_stat(&player_name, &player_block, &player_health);
    for (skill_name, damage) in skills.iter() {
        println!(
            "{}'s receiving offensive skill {} ({})",
            player_name.name, skill_name.name, damage.value
        );
        player_health.value -= damage.value - player_block.value;
        match damage.value > player_block.value {
            true => player_block.value = 0,
            false => player_block.value -= damage.value,
        }
    }
    print_player_stat(&player_name, &player_block, &player_health);
    println!("====================");
}
fn calc_heal(
    mut player: Query<(&LabelName, &Block, &mut Health, &MaxHealth), With<Player>>,
    skills: Query<(&LabelName, &Heal), (With<Skill>, Without<Player>)>,
) {
    let (player_name, player_block, mut player_health, player_max_health) = player.single_mut();
    print_player_stat(&player_name, &player_block, &player_health);
    for (skill_name, heal) in skills.iter() {
        println!(
            "{}'s casting healing skill {} ({})",
            player_name.name, skill_name.name, heal.value
        );

        // can only heal when alive
        // NOTE: potential refactor
        if player_health.value > 1 {
            player_health.value = match player_health.value + heal.value > player_max_health.value {
                true => player_max_health.value,
                false => player_health.value + heal.value,
            };
        }
    }
    print_player_stat(&player_name, &player_block, &player_health);
    println!("====================");
}

fn print_player_stat(name: &LabelName, current_block: &Block, current_health: &Health) {
    println!(
        "system: {}'s health: {}, block: {}",
        name.name, current_health.value, current_block.value
    );
}
