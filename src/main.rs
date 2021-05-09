mod components;
mod constants;
mod resources;
mod systems;

use bevy::prelude::*;
use constants::{BG_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH};
use resources::Sprites;
use systems::player;

fn main() {
    let window = WindowDescriptor {
        title: "Industrial".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        ..Default::default()
    };

    let sprites = Sprites::new();
    let mut app = App::build();

    app.insert_resource(BG_COLOR)
        .insert_resource(window)
        .insert_resource(sprites)
        .add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(player::setup.system())
        .add_system(player::action.system())
        .add_system(player::animation.system())
        .run();
}
