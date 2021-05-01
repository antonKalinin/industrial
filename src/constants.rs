use bevy::{prelude::*, render::pass::ClearColor};

pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 360.0;

pub const GRAVITY: f32 = 18.0;
pub const PLAYER_HORIZONTAL_SPEED: f32 = 5.0;
pub const PLAYER_INITIAL_VERTICAL_SPEED: f32 = 7.0;

pub const BG_COLOR: ClearColor = ClearColor(Color::rgb(0.114, 0.129, 0.176));
