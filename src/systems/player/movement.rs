use bevy::prelude::*;

use crate::components::{Player, PlayerState};
use crate::constants::{GRAVITY, PLAYER_HORIZONTAL_SPEED, PLAYER_INITIAL_VERTICAL_SPEED};

pub fn movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<(&mut Player, &mut Transform)>,
) {
  for (mut player, mut player_transform) in player_query.iter_mut() {
    let player_jumping = player.state == PlayerState::InAir || player.state == PlayerState::Landing;

    if keyboard_input.pressed(KeyCode::Right) && !player_jumping {
      player.velocity.x = PLAYER_HORIZONTAL_SPEED;
      player_transform.rotation = Quat::from_rotation_y(0.0);
      player.state = PlayerState::Running;
    }

    if keyboard_input.pressed(KeyCode::Left) && !player_jumping {
      player.velocity.x = -PLAYER_HORIZONTAL_SPEED;
      player_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
      player.state = PlayerState::Running;
    }

    if keyboard_input.pressed(KeyCode::Up) && !player_jumping {
      player.velocity.y = PLAYER_INITIAL_VERTICAL_SPEED;
      player.state = PlayerState::InAir;
    }

    if keyboard_input.just_released(KeyCode::Right) || keyboard_input.just_released(KeyCode::Left) {
      player.velocity.x = 0.0;

      if player.state != PlayerState::InAir {
        player.state = PlayerState::Idle;
      }
    }

    // player is constantly affected by gravity
    player.velocity.y -= GRAVITY * time.delta_seconds();

    // for testing jumping & gravity
    if (player_transform.translation.y + player.velocity.y) < 0.0 {
      player.velocity.y = 0.0;

      if player.state == PlayerState::InAir {
        player.state = PlayerState::Landing;
      }
    }

    let player_next_translation = player_transform.translation + player.velocity;

    player_transform.translation = player_next_translation;
  }
}
