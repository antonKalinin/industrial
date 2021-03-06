use bevy::prelude::{Handle, Texture, TextureAtlas, Vec2};
use bevy::sprite::Rect;

pub fn from_grid_with_offset(
  texture: Handle<Texture>,
  tile_size: Vec2,
  offset: Vec2,
  rows: usize,
  columns: usize,
) -> TextureAtlas {
  let mut sprites = Vec::new();

  for y in 0..columns {
    for x in 0..rows {
      let rect_min = Vec2::new(
        offset.x + tile_size.x * x as f32,
        offset.y + tile_size.y * y as f32,
      );

      sprites.push(Rect {
        min: rect_min,
        max: Vec2::new(rect_min.x + tile_size.x, rect_min.y + tile_size.y),
      })
    }
  }

  TextureAtlas {
    size: Vec2::new(512.0, 512.0),
    textures: sprites,
    texture,
    texture_handles: None,
  }
}

pub fn from_tiles(
  texture: Handle<Texture>,
  tile_size: Vec2,
  tiles_x0y0: Vec<Vec2>,
) -> TextureAtlas {
  let mut sprites = Vec::new();

  for x0y0 in tiles_x0y0 {
    sprites.push(Rect {
      min: x0y0,
      max: Vec2::new(x0y0.x + tile_size.x, x0y0.y + tile_size.y),
    })
  }

  TextureAtlas {
    size: Vec2::new(512.0, 512.0),
    textures: sprites,
    texture,
    texture_handles: None,
  }
}
