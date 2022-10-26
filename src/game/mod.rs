use bevy::prelude::*;
use iyes_loopless::prelude::*;
pub struct GamePlugin;

/// OutOfCombat: when character is in world, can move
/// InCombat: when character is in combat, can't move
/// InTurn: character's turn, skill UI visible
/// NotInTurn: after character's turn, skill UI invisible
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    InCombat,
    OutOfCombat,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CombatState {
    InTurn,
    NotInTurn,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // enum type must be unique (only 1 GameState here)
        // default init for state value, TODO: handle change with event
        app.add_loopless_state(GameState::OutOfCombat)
            .add_loopless_state(CombatState::InTurn)
            .add_system(hello_combat.run_in_state(GameState::InCombat))
            .add_system(hello.run_in_state(GameState::OutOfCombat));
    }
}
struct ComesPlayerTurn;
struct ComesEnemyTurn;
struct PlayerSelectedSkill;
struct EnemySelectedSkill;
// TODO: render skill list when in InTurn state
// TODO: transform into event-based systems with buttons
fn hello() {
    // frame-based system
    info!("hello from GamePlugin");
}
fn is_in_combat(game_state: Res<GameState>) -> bool {
    *game_state == GameState::InCombat
}
fn is_in_env(game_state: Res<GameState>) -> bool {
    *game_state == GameState::OutOfCombat
}

fn hello_combat() {
    info!("hello from GamePlugin but you're in combat!");
}
