use bevy::prelude::*;
use crate::ecs::component::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::new().with_system(logic_input_movement));
    }
}

/// Transform sprite based on position in the last frame
fn logic_input_movement(
    time: Res<Time>,
    mut sprite_pos: Query<(&mut IsMoving, &mut Transform), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut is_moving, mut transform) in &mut sprite_pos {
        // let mut dir: Direction = Direction::Down; // facing the screen
        let _dir: Direction = match input.get_pressed().next() {
            Some(KeyCode::W) => {
                is_moving.0 = true;
                transform.translation.y += 150. * time.delta_seconds();
                Direction::Up
            }
            Some(KeyCode::R) => {
                is_moving.0 = true;
                transform.translation.y -= 150. * time.delta_seconds();
                Direction::Down
            }
            Some(KeyCode::A) => {
                is_moving.0 = true;
                transform.translation.x -= 150. * time.delta_seconds();
                Direction::Left
            }
            Some(KeyCode::S) => {
                is_moving.0 = true;
                transform.translation.x += 150. * time.delta_seconds();
                Direction::Right
            }
            _ => {
                is_moving.0 = false;
                Direction::Down
            } // stops moving
        };
    }
}

/// CP, movement direction, should(?) be linked with keyboard input
#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}