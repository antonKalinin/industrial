use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum GameStatus {
  Started,
  Finished,
  Paused,
}

#[derive(Debug)]
pub struct GameState {
  pub game_status: GameStatus,
}

impl Default for GameState {
  fn default() -> Self {
    Self {
      game_status: GameStatus::Paused,
    }
  }
}

pub struct Sprites {
  library: HashMap<String, Handle<TextureAtlas>>,
}

impl Sprites {
  pub fn new() -> Sprites {
    Sprites {
      library: HashMap::new(),
    }
  }

  pub fn add(&mut self, key: String, atlas_handle: Handle<TextureAtlas>) {
    self.library.insert(key, atlas_handle);
  }

  pub fn get(&self, key: &str) -> Option<&Handle<TextureAtlas>> {
    self.library.get(key)
  }
}
