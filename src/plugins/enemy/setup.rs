use crate::components::{Enemy, EnemyState};
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

  let robot_tiles = vec![
    Vec2::new(96.0, 256.0),
    Vec2::new(80.0, 256.0),
    Vec2::new(64.0, 256.0),
    Vec2::new(48.0, 256.0),
  ];

  sprites.add("robot".to_string(), [0, robot_tiles.len()]);

  for tile in robot_tiles {
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

  let enemy = Enemy {
    size: Vec2::new(0.0, 0.0),
    velocity: Vec3::new(0.0, 0.0, 0.0),
    state: PlayerState::Idle,
  };

  commands
    .spawn_bundle(SpriteSheetBundle {
      texture_atlas: texture_atlas_handle.clone(),
      transform: Transform::from_scale(Vec3::splat(2.0)),
      ..Default::default()
    })
    .insert(enemy)
    .insert(Timer::from_seconds(0.1, true));
}
