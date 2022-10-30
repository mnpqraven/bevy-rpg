use bevy::prelude::*;

pub mod component;
pub mod sprites;
pub mod bundle;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(sprites::SpritePlugin);
    }
}