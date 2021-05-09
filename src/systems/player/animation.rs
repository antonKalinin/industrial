use crate::components::{Player, PlayerState};
use crate::resources::Sprites;
use bevy::prelude::*;

pub fn animation(
  time: Res<Time>,
  sprites: Res<Sprites>,
  mut player_query: Query<(&mut Player, &mut Timer, &mut TextureAtlasSprite)>,
) {
  for (mut player, mut timer, mut sprite) in player_query.iter_mut() {
    timer.tick(time.delta());

    if timer.finished() {
      let start_end: [usize; 2];

      match player.state {
        PlayerState::Idle => start_end = *sprites.get("player_idle").unwrap(),
        PlayerState::InAir => start_end = *sprites.get("player_air").unwrap(),
        PlayerState::Landing => start_end = *sprites.get("player_land").unwrap(),
        PlayerState::Running => start_end = *sprites.get("player_run").unwrap(),
        PlayerState::Fighting => start_end = *sprites.get("player_fight").unwrap(),
        PlayerState::Shooting => start_end = *sprites.get("player_shoot").unwrap(),
      }

      if sprite.index < start_end[0] as u32 || sprite.index >= start_end[1] as u32 {
        sprite.index = start_end[0] as u32;
      } else {
        sprite.index += 1;

        if sprite.index >= start_end[1] as u32 {
          // It was final frame of current animation.
          // So if current animation should be played only one time,
          // it is a right moment to switch player to another state
          match player.state {
            PlayerState::Landing => player.state = PlayerState::Idle,
            PlayerState::Fighting => player.state = PlayerState::Idle,
            PlayerState::Shooting => player.state = PlayerState::Idle,
            _ => {}
          }
        }
      }
    }
  }
}
