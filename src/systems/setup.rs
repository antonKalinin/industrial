use super::super::components::Player;
use super::super::resources::Sprites;
use super::super::utils::texture_atlas;
use bevy::prelude::*;

pub fn setup(
  mut commands: Commands,
  mut sprites: ResMut<Sprites>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture = asset_server.load("industrial.png");
  let tile_size = Vec2::new(16.0, 16.0);

  let run_texture_atlas =
    texture_atlas::from_grid_with_offset(texture.clone(), tile_size, Vec2::new(0.0, 272.0), 8, 1);
  let idle_texture_atlas =
    texture_atlas::from_grid_with_offset(texture.clone(), tile_size, Vec2::new(0.0, 256.0), 3, 1);

  let run_texture_atlas_handle = texture_atlases.add(run_texture_atlas);
  let idle_texture_atlas_handle = texture_atlases.add(idle_texture_atlas);

  let player = Player {
    size: Vec2::new(0.0, 0.0),
    velocity: Vec3::new(0.0, 0.0, 0.0),
    initial_position: Vec3::new(0.0, 0.0, 0.0),
    is_in_air: false,
  };

  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands
    .spawn_bundle(SpriteSheetBundle {
      texture_atlas: idle_texture_atlas_handle.clone(),
      transform: Transform::from_scale(Vec3::splat(2.0)),
      ..Default::default()
    })
    .insert(player)
    .insert(Timer::from_seconds(0.1, true));

  sprites.add("player_run".to_string(), run_texture_atlas_handle);
  sprites.add("player_idle".to_string(), idle_texture_atlas_handle);
}
