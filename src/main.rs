mod game;

use crate::game::GamePlugin;
use bevy::{prelude::*, render::camera::ScalingMode};

pub const SCREEN_WIDTH: f32 = 768.0;
pub const SCREEN_HEIGHT: f32 = 1024.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".into(),
                        resolution: (720.0, 960.0).into(),
                        resizable: false,
                        // mode: WindowMode::Fullscreen,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: SCREEN_WIDTH,
        min_height: SCREEN_HEIGHT,
    };
    commands.spawn(camera);
    // commands.spawn(Camera2dBundle::default());
}
