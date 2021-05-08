use crate::components::{Player, PlayerState};
use crate::resources::Sprites;
use bevy::prelude::*;
use bevy::sprite::Rect;

pub fn setup(
  mut commands: Commands,
  mut sprites: ResMut<Sprites>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let texture = asset_server.load("industrial.png");
  let tile_size = Vec2::new(16.0, 16.0);
  let mut tiles = Vec::new();
  let mut textures = Vec::new();

  let air_tiles = vec![Vec2::new(0.0, 256.0)];

  let land_tiles = vec![
    Vec2::new(96.0, 256.0),
    Vec2::new(80.0, 256.0),
    Vec2::new(64.0, 256.0),
    Vec2::new(48.0, 256.0),
  ];

  let idle_tiles = vec![
    Vec2::new(16.0, 256.0),
    Vec2::new(16.0, 256.0),
    Vec2::new(16.0, 256.0),
    Vec2::new(0.0, 256.0),
    Vec2::new(0.0, 256.0),
    Vec2::new(0.0, 256.0),
    Vec2::new(16.0, 256.0),
    Vec2::new(16.0, 256.0),
    Vec2::new(16.0, 256.0),
    Vec2::new(32.0, 256.0),
    Vec2::new(32.0, 256.0),
    Vec2::new(32.0, 256.0),
  ];

  let run_tiles = vec![
    Vec2::new(0.0, 272.0),
    Vec2::new(16.0, 272.0),
    Vec2::new(32.0, 272.0),
    Vec2::new(48.0, 272.0),
    Vec2::new(64.0, 272.0),
    Vec2::new(80.0, 272.0),
    Vec2::new(96.0, 272.0),
    Vec2::new(112.0, 272.0),
  ];

  let tiles_with_keys = vec![
    ("player_air", air_tiles),
    ("player_run", run_tiles),
    ("player_idle", idle_tiles),
    ("player_land", land_tiles),
  ];

  for (key, mut animation_tiles) in tiles_with_keys {
    sprites.add(
      key.to_string(),
      [tiles.len(), tiles.len() + animation_tiles.len() - 1],
    );
    tiles.append(&mut animation_tiles);
  }

  for tile in tiles {
    textures.push(Rect {
      min: tile,
      max: Vec2::new(tile.x + tile_size.x, tile.y + tile_size.y),
    })
  }

  let texture_atlas = TextureAtlas {
    size: Vec2::new(512.0, 512.0),
    textures,
    texture,
    texture_handles: None,
  };

  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  let player = Player {
    size: Vec2::new(0.0, 0.0),
    velocity: Vec3::new(0.0, 0.0, 0.0),
    state: PlayerState::Idle,
  };

  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands
    .spawn_bundle(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle.clone(),
      transform: Transform::from_scale(Vec3::splat(2.0)),
      ..Default::default()
    })
    .insert(player)
    .insert(Timer::from_seconds(0.10, true));
}
