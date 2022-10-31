use bevy::prelude::*;

pub struct EnvUIPlugin;

impl Plugin for EnvUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(hello_world);
    }
}

fn hello_world() {
    {}
}