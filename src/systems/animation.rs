use super::super::components::Player;
use super::super::resources::Sprites;
use bevy::prelude::*;

pub fn animation(
  time: Res<Time>,
  sprites: Res<Sprites>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut player_query: Query<(
    &Player,
    &mut Timer,
    &mut TextureAtlasSprite,
    &mut Handle<TextureAtlas>,
  )>,
) {
  for (player, mut timer, mut sprite, mut texture_atlas_handle) in player_query.iter_mut() {
    timer.tick(time.delta());

    if player.velocity.x != 0.0 {
      if let Some(player_run) = sprites.get("player_run") {
        texture_atlas_handle.id = player_run.id;
      }
    } else {
      if let Some(player_idle) = sprites.get("player_idle") {
        texture_atlas_handle.id = player_idle.id;
      }
    }

    if timer.finished() {
      let texture_atlas = texture_atlases.get(texture_atlas_handle.clone()).unwrap();
      sprite.index = ((sprite.index as usize + 1) % texture_atlas.len()) as u32;
    }
  }
}
