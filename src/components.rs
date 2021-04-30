use bevy::prelude::*;

pub struct Player {
  pub size: Vec2,
  pub velocity: Vec3,
  pub initial_position: Vec3,
  pub is_in_air: bool,
}
