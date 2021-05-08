use std::collections::HashMap;

#[derive(Debug)]
pub enum GameState {
  Started,
  Finished,
  Paused,
}

pub struct Sprites {
  store: HashMap<String, [usize; 2]>,
}

impl Sprites {
  pub fn new() -> Sprites {
    Sprites {
      store: HashMap::new(),
    }
  }

  pub fn add(&mut self, key: String, atlas_handle_id: [usize; 2]) {
    self.store.insert(key, atlas_handle_id);
  }

  pub fn get(&self, key: &str) -> Option<&[usize; 2]> {
    self.store.get(key)
  }
}
