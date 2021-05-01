use bevy::prelude::*;

use super::super::components::Player;
use super::super::constants::{GRAVITY, PLAYER_HORIZONTAL_SPEED, PLAYER_INITIAL_VERTICAL_SPEED};

pub fn movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<(&mut Player, &mut Transform)>,
) {
  for (mut player, mut player_transform) in player_query.iter_mut() {
    if keyboard_input.pressed(KeyCode::Up) && !player.is_in_air {
      player.velocity.y = PLAYER_INITIAL_VERTICAL_SPEED;
      player.is_in_air = true;
    }

    if keyboard_input.pressed(KeyCode::Right) && !player.is_in_air {
      player.velocity.x = PLAYER_HORIZONTAL_SPEED;
      player_transform.rotation = Quat::from_rotation_y(0.0);
    }

    if keyboard_input.pressed(KeyCode::Left) && !player.is_in_air {
      player.velocity.x = -PLAYER_HORIZONTAL_SPEED;
      player_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    }

    if keyboard_input.just_released(KeyCode::Right) || keyboard_input.just_released(KeyCode::Left) {
      player.velocity.x = 0.0;
    }

    // player is constantly affected by gravity
    player.velocity.y -= GRAVITY * time.delta_seconds();

    // for testing jumping & gravity
    if (player_transform.translation.y + player.velocity.y) < 0.0 {
      player.velocity.y = 0.0;
      player.is_in_air = false;
    }

    let player_next_translation = player_transform.translation + player.velocity;

    player_transform.translation = player_next_translation;
  }
}
