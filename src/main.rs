use bevy::prelude::*;
fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "bevy-rpg".to_string(),
            ..default()
        })
        .add_startup_system_set( // unit archetype
            SystemSet::new()
                .with_system(spawn_player)
                .with_system(spawn_enemy),
        )
        .add_startup_system_set( // skil larchetype
            SystemSet::new()
                .with_system(spawn_skill_basic_attack)
                .with_system(spawn_skill_basic_block)
                .with_system(spawn_skill_basic_heal),
        )
        .add_system(get_player_name)
        .add_system_set(
            SystemSet::new()
                .with_system(calc_block)
                .with_system(calc_damage.after(calc_block))
                .with_system(calc_heal),
        )
        .run();
}

// COMPONENT ==================================================================
/// CP
/// User-controlled component
#[derive(Component, Debug)]
struct Player;
/// CP
#[derive(Component)]
struct Enemy;

/// CP
#[derive(Component, Clone)]
struct Name {
    name: String,
}

/// CP
#[derive(Component)]
struct Skill;

// STATS ============
/// CP
#[derive(Component, Clone)]
struct Health {
    value: i32,
}
/// CP
#[derive(Component, Clone)]
struct MaxHealth {
    value: i32,
}
/// CP
// #[derive(Component)]
// struct Mana(i32);
/// CP
#[derive(Component)]
struct Damage {
    value: i32,
}
/// CP
#[derive(Component, Clone)]
struct Block {
    value: i32,
}
impl Default for Block {
    fn default() -> Self {
        Self { value: 0 }
    }
}
/// CP
#[derive(Component)]
struct Heal {
    value: i32,
}

// ENTITY =====================================================================
// RESOURCE ===================================================================
#[derive(Bundle)]
struct CombatStatBundle {
    base_health: Health,
    base_damage: Damage,
    base_block: Block,
}
// is this actually correct, not very sure
#[derive(Bundle)]
struct CharacterBundle {
    name: Name,
    current_health: Health,
    current_block: Block,
}

// SYSTEM =====================================================================
fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Name {
            name: "Othi".to_string(),
        })
        .insert(Health { value: 100 })
        .insert(MaxHealth { value: 100 })
        .insert(Block::default());
}
fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn()
        .insert(Enemy)
        .insert(Name {
            name: "training dummy".to_string(),
        })
        .insert(Health { value: 40 })
        .insert(MaxHealth { value: 40 })
        .insert(Block::default());
}

fn spawn_skill_basic_attack(mut commands: Commands) {
    commands
        .spawn()
        .insert(Skill)
        .insert(Name {
            name: "Attack".to_string(),
        })
        .insert(Damage { value: 7 });
}
fn spawn_skill_basic_block(mut commands: Commands) {
    commands
        .spawn()
        .insert(Skill)
        .insert(Name {
            name: "Block".to_string(),
        })
        .insert(Block { value: 5 });
}
fn spawn_skill_basic_heal(mut commands: Commands) {
    commands
        .spawn()
        .insert(Skill)
        .insert(Name {
            name: "Bandaid".to_string(),
        })
        .insert(Heal { value: 5 });
}
fn get_player_name(mut players: Query<(&Health, &Name, Option<&Player>)>) {
    for (health, name, player) in players.iter_mut() {
        println!(
            "STARTUP: entity {} has {} health points",
            name.name, health.value
        );
        if let Some(_) = player {
            // user controlled
            println!(
                "STARTUP: entity {} is special because it is controlled by a player",
                name.name
            );
        }
    }
}
fn calc_block(
    mut player: Query<(&Name, &Health, &mut Block), With<Player>>,
    skills: Query<(&Name, &Block), (With<Skill>, Without<Player>)>,
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
    mut player: Query<(&Name, &mut Block, &mut Health), With<Player>>,
    skills: Query<(&Name, &Damage), (With<Skill>, Without<Player>)>,
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
    mut player: Query<(&Name, &Block, &mut Health, &MaxHealth), With<Player>>,
    skills: Query<(&Name, &Heal), (With<Skill>, Without<Player>)>,
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

fn print_player_stat(name: &Name, current_block: &Block, current_health: &Health) {
    println!(
        "system: {}'s health: {}, block: {}",
        name.name, current_health.value, current_block.value
    );
}
fn _game_over() {
    // triggers GameOverEvent if hp drops to 0
}
fn _test_loop() {
    // player getting attacked by "Attack"
    // automatically casts "Bandaid" when health is < 50%
    // automatically casts "Block" when health is < 25%
}
