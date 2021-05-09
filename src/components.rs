use bevy::prelude::*;

#[derive(PartialEq)]
pub enum PlayerState {
  Idle,
  InAir,
  Landing,
  Running,
  Fighting,
  Shooting,
}

pub struct Player {
  pub size: Vec2,
  pub velocity: Vec3,
  pub state: PlayerState,
}
