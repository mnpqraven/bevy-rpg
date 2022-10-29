use bevy::prelude::*;
use crate::game::component::{Direction, *};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::new().with_system(logic_input_movement));
    }
}

/// transform sprite based on position in the last frame
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
