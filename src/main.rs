use bevy::prelude::*;
fn main() {
    App::new()
    // .add_plugins(DefaultPlugins)
    .insert_resource(WindowDescriptor {
        title: "bevy-rpg".to_string(),
        ..default()
    })
    .add_startup_system(spawn_player)
    .add_startup_system_set(
        SystemSet::new()
        .with_system(spawn_skill_basic_attack)
        .with_system(spawn_skill_basic_block)
        .with_system(spawn_skill_basic_heal),
    )
    .add_system(get_player_name)
    .add_system(damage_calculation)
    .run();
}

// COMPONENT ==================================================================
/// CP
/// User-controlled component
#[derive(Component)]
#[derive(Debug)]
struct Player;
/// CP
#[derive(Component)]
struct Enemy(String);

/// CP
#[derive(Component)]
struct Name{ name: String }

/// CP
#[derive(Component)]
struct Skill;

// STATS ============
/// CP
#[derive(Component)]
struct Health{ hp: i32 }
/// CP
// #[derive(Component)]
// struct Mana(i32);
/// CP
#[derive(Component)]
struct Damage(i32);
/// CP
#[derive(Component)]
struct Block(i32);
/// CP
#[derive(Component)]
struct Heal(i32);

// ENTITY =====================================================================
// RESOURCE ===================================================================
#[derive(Bundle)]
struct CombatStatBundle {
    base_health: Health,
    base_damage: Damage,
    base_block: Block
}
struct SelectedUnit;
// SYSTEM =====================================================================
fn spawn_player(mut commands: Commands) {
    commands.spawn()
    .insert(Player)
    .insert(Name {name: "Othi".to_string()})
    .insert(Health {hp: 100});
}
fn spawn_skill_basic_attack(mut commands: Commands) {
    commands.spawn()
    .insert(Skill)
    .insert(Name {name: "Attack".to_string()})
    .insert(Damage (7));
}
fn spawn_skill_basic_block(mut commands: Commands) {
    commands.spawn()
    .insert(Skill)
    .insert(Name {name: "Block".to_string()})
    .insert(Block (5));
}
fn spawn_skill_basic_heal(mut commands: Commands) {
    commands.spawn()
    .insert(Skill)
    .insert(Name {name: "Bandaid".to_string()})
    .insert(Heal (5));
}
fn get_player_name(mut players: Query<(&Health, &Name, Option<&Player>)>) {
        for (health, name, player) in players.iter_mut() {
            println!("STARTUP: entity {} has {} health points", name.name, health.hp);
            if let Some(player) = player {
                // user controlled
                println!("STARTUP: entity {} is special because it is controlled by a player", name.name);
            }
        }
}
fn damage_calculation(
    // mut commands: Commands,
    mut query: Query<&mut Health, With<Player>>,
    // skills: Query<(&Name, &Damage, &Block, &Heal), With<Skill>>) {
    skills: Query<(&Name, Option<&Damage>, Option<&Block>, Option<&Heal>), With<Skill>>) {
        let mut player = query.single_mut();
        println!("s: player hp {:?}", player.hp);
        for (name, damage, block, heal) in skills.iter() {
            print!("Skill: {} ||| ", name.name);
            if damage.is_some(){
                println!("Damage: {}", damage.unwrap().0);
            }
            if block.is_some(){
                println!("Block: {}", block.unwrap().0);
            }
            if heal.is_some(){
                println!("Heal: {}", heal.unwrap().0);
            }
        }
        // TODO: actual calculation based on this
        player.hp += 10;
        println!("s: player hp {:?}", player.hp);
}
fn test_loop() {
    // player getting attacked by "Attack"
    // automatically casts "Bandaid" when health is < 50%
    // automatically casts "Block" when health is < 25%
}