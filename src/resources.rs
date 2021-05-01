use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum GameState {
  Started,
  Finished,
  Paused,
}

pub struct Sprites {
  collection: HashMap<String, Handle<TextureAtlas>>,
}

impl Sprites {
  pub fn new() -> Sprites {
    Sprites {
      collection: HashMap::new(),
    }
  }

  pub fn add(&mut self, key: String, atlas_handle_id: Handle<TextureAtlas>) {
    self.collection.insert(key, atlas_handle_id);
  }

  pub fn get(&self, key: &str) -> Option<&Handle<TextureAtlas>> {
    self.collection.get(key)
  }
}
