use bevy::{log::LogSettings, prelude::*};

use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use ecs::component::{Channel, LabelName};

mod combat;
mod ecs;
mod environment;
mod game;
mod skills;
mod ui;

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
        // bevy plugins
        .add_plugins(DefaultPlugins)
        // user modules
        .add_plugin(skills::SkillPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(combat::CombatPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(environment::EnvironmentPlugin)
        // 3rd party plugins
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<LabelName>()
        .register_inspectable::<Channel>()
        //
        .add_startup_system_set(
            SystemSet::new()
                .with_system(x11_scale)
                .with_system(spawn_camera),
        )
        .add_system(bevy::window::close_on_esc)
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

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
