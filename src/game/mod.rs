use bevy::prelude::*;

pub mod sprites;
pub mod bundle;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(sprites::SpritePlugin);
    }
}

/// Despawns all entities with component T
pub fn despawn_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
