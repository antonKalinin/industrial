mod systems;

use bevy::prelude::*;
use systems::{action, animation, setup};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_startup_system(setup.system())
      .add_system(action.system())
      .add_system(animation.system());
  }
}
