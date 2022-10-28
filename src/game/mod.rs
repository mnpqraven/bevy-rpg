use bevy::prelude::*;
use iyes_loopless::prelude::*;

// NOTE: other non-mod.rs modules (folders included) in the folder here
pub mod component;
pub mod sprites;

pub struct GamePlugin;


impl Plugin for GamePlugin {
    fn build(&self, _app: &mut App) {
    }
}