use bevy::prelude::*;

mod globals;
mod camera;
mod vectors;
mod board;
mod graphics;
mod player;
mod pieces;
mod input;
mod props;
mod events;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((board::BoardPlugin, graphics::GraphicsPlugin, player::PlayerPlugin, props::PropsPlugin, input::InputPlugin, pieces::PiecesPlugin))
        .add_systems(Startup, camera::spawn_camera)
        .run();
}
