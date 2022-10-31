use bevy::{log::LogSettings, prelude::*};

use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};
use game::component::LabelName;

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
        .add_plugin(crate::combat::CombatPlugin)
        .add_plugin(crate::game::GamePlugin)
        .add_plugin(crate::environment::EnvironmentPlugin)
        // 3rd party plugins
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<LabelName>()
        .add_system(bevy::window::close_on_esc)
        .run();
}
