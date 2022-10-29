use bevy::{log::LogSettings, prelude::*};

// modules
use game::component::*; // fix bevy name conflict
use game::sprites::{load_ascii, spawn_player};

mod combat;
mod environment;
mod menu;
mod game;
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
        .add_plugin(crate::environment::EnvironmentPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_startup_system_set_to_stage(
            StartupStage::PostStartup,
            SystemSet::new()
                .with_system(spawn_player)
                // TODO: conditional spawning later
                .with_system(spawn_enemy),
        )
        .run();
}

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