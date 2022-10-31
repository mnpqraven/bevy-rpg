use bevy::{log::LogSettings, prelude::*};

use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use game::component::LabelName;

mod combat;
mod environment;
mod game;
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
        .add_plugin(crate::combat::CombatPlugin)
        .add_plugin(crate::game::GamePlugin)
        .add_plugin(crate::environment::EnvironmentPlugin)
        // 3rd party plugins
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<LabelName>()
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(x11_scale)
        .run();
}
/// 1x scale factor for small screen (debug)
fn x11_scale(mut windows: ResMut<Windows>) {
    if std::env::consts::OS.eq("linux") {
        for window in windows.iter_mut() {
            window.set_scale_factor_override(Some(1.));
        }
    }
}
