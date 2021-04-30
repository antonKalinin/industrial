use bevy::prelude::*;

use super::super::components::Player;
use super::super::constants::PLAYER_HORIZONTAL_SPEED;

pub fn movement(
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<(&mut Player, &mut Transform)>,
) {
  for (mut player, mut player_transform) in player_query.iter_mut() {
    if keyboard_input.pressed(KeyCode::Right) {
      player.velocity.x = PLAYER_HORIZONTAL_SPEED;
      player_transform.rotation = Quat::from_rotation_y(0.0);
    }

    if keyboard_input.pressed(KeyCode::Left) {
      player.velocity.x = -PLAYER_HORIZONTAL_SPEED;
      player_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    }

    if keyboard_input.just_released(KeyCode::Right) || keyboard_input.just_released(KeyCode::Left) {
      player.velocity.x = 0.0;
    }

    let player_next_translation = player_transform.translation + player.velocity;

    player_transform.translation = player_next_translation;
  }
}
