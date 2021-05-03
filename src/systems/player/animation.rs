use crate::components::{Player, PlayerState};
use crate::resources::Sprites;
use bevy::prelude::*;

pub fn animation(
  time: Res<Time>,
  sprites: Res<Sprites>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut player_query: Query<(
    &mut Player,
    &mut Timer,
    &mut TextureAtlasSprite,
    &mut Handle<TextureAtlas>,
  )>,
) {
  for (mut player, mut timer, mut sprite, mut texture_atlas_handle) in player_query.iter_mut() {
    timer.tick(time.delta());

    match player.state {
      PlayerState::Idle => {
        texture_atlas_handle.id = sprites.get("player_idle").unwrap().id;
      }
      PlayerState::Running => {
        texture_atlas_handle.id = sprites.get("player_run").unwrap().id;
      }
      PlayerState::Jumping => {
        texture_atlas_handle.id = sprites.get("player_jump").unwrap().id;
      }
      PlayerState::InAir => {
        texture_atlas_handle.id = sprites.get("player_air").unwrap().id;
      }
      PlayerState::Landing => {
        texture_atlas_handle.id = sprites.get("player_land").unwrap().id;
      }
    }

    if timer.finished() {
      let texture_atlas = texture_atlases.get(texture_atlas_handle.clone()).unwrap();
      sprite.index = ((sprite.index as usize + 1) % texture_atlas.len()) as u32;

      if (sprite.index as usize + 1) == texture_atlas.len() {
        match player.state {
          PlayerState::Jumping => player.state = PlayerState::InAir,
          PlayerState::Landing => player.state = PlayerState::Idle,
          _ => {}
        }
      }
    }
  }
}
