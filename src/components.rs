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

#[derive(PartialEq)]
pub enum EnemyState {
  Idle,
  Running,
}

pub struct Enemy {
  pub size: Vec2,
  pub velocity: Vec3,
  pub state: EnemyState,
}
