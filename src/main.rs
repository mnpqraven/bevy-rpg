use bevy::{log::LogSettings, prelude::*};

// modules
mod game;
pub mod combat;
use game::component::{Direction, *}; // fix bevy name conflict
use game::sprites::{load_ascii, spawn_player};
mod menu;
mod skills;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "bevy-rpg".to_string(),
            width: 1280.,
            height: 768.,
            ..default()
        })
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,othirpg=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(crate::skills::SkillPlugin)
        .add_plugin(crate::menu::MenuPlugin)
        .add_plugin(crate::game::GamePlugin)
        .add_plugin(crate::combat::CombatPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_system(bevy::window::close_on_esc)
        .add_startup_system_set_to_stage(
            StartupStage::PostStartup,
            SystemSet::new()
                // unit archetype
                .with_system(spawn_player)
                .with_system(spawn_enemy),
        )
        .add_system_set(SystemSet::new().with_system(logic_input_movement))
        // TODO: turn base queueing
        // .add_system(get_player_name)
        // .add_system_set(
        //     SystemSet::new()
        //         .with_system(calc_block)
        //         .with_system(calc_damage.after(calc_block))
        //         .with_system(calc_heal),
        // )
        .run();
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
    name: LabelName,
    current_health: Health,
    current_block: Block,
}

// SYSTEM =====================================================================
fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn()
        .insert(Enemy)
        .insert(LabelName {
            name: "training dummy".to_string(),
        })
        .insert(Health { value: 40 })
        .insert(MaxHealth { value: 40 })
        .insert(Mana { value: 100})
        .insert(Block::default());
}

fn _get_player_name(mut players: Query<(&Health, &LabelName, Option<&Player>)>) {
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

fn _game_over() {
    // triggers GameOverEvent if hp drops to 0
}
fn _test_loop() {
    // player getting attacked by "Attack"
    // automatically casts "Bandaid" when health is < 50%
    // automatically casts "Block" when health is < 25%
}

/// transform sprite based on position in the last frame
/// TODO: refactor
fn logic_input_movement(
    time: Res<Time>,
    mut sprite_pos: Query<(&mut IsMoving, &mut Transform), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut is_moving, mut transform) in &mut sprite_pos {
        // let mut dir: Direction = Direction::Down; // facing the screen
        let dir: crate::game::component::Direction = match input.get_pressed().next() {
            Some(KeyCode::W) => {
                is_moving.0 = true;
                Direction::Up
            }
            Some(KeyCode::R) => {
                is_moving.0 = true;
                Direction::Down
            }
            Some(KeyCode::A) => {
                is_moving.0 = true;
                Direction::Left
            }
            Some(KeyCode::S) => {
                is_moving.0 = true;
                Direction::Right
            }
            _ => {
                is_moving.0 = false;
                Direction::Down
            } // stops moving
        };
        if is_moving.0 {
            match dir {
                Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
                Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
                Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
                Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            }
        }
    }
}
