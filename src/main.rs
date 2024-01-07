mod game;

use crate::game::GamePlugin;
use bevy::{prelude::*, render::camera::ScalingMode};

pub const SCREEN_WIDTH: f32 = 288.0;
pub const SCREEN_HEIGHT: f32 = 512.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".into(),
                        resolution: (SCREEN_WIDTH * 2.0, SCREEN_HEIGHT * 2.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<AppState>()
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    // MainMenu,
    #[default]
    InGame,
    GameOver,
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: SCREEN_WIDTH,
        min_height: SCREEN_HEIGHT,
    };
    commands.spawn(camera);
}
