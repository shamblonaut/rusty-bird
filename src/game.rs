mod faby;
mod pipes;

use crate::{
    game::{
        faby::{drop_faby, spawn_faby},
        pipes::{move_columns, spawn_column, spawn_columns},
    },
    SCREEN_HEIGHT, SCREEN_WIDTH,
};
use bevy::prelude::*;

use self::pipes::PipeSpawnEvent;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeSpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )));
        app.add_event::<PipeSpawnEvent>();
        app.add_systems(Startup, spawn_faby);
        app.add_systems(Startup, spawn_ground);
        app.add_systems(Update, spawn_columns);
        app.add_systems(Update, spawn_column);
        app.add_systems(Update, drop_faby);
        app.add_systems(Update, move_columns);
    }
}

pub const GROUND_HEIGHT: f32 = SCREEN_HEIGHT / 5.0;

#[derive(Resource)]
pub struct PipeSpawnTimer(Timer);

#[derive(Component)]
pub struct Ground;

fn spawn_ground(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: (-SCREEN_HEIGHT / 2.0) + (GROUND_HEIGHT / 2.0),
                    z: 2.0,
                },
                scale: Vec3 {
                    x: SCREEN_WIDTH,
                    y: GROUND_HEIGHT,
                    z: 1.0,
                },
                ..default()
            },
            ..default()
        },
        Ground,
        Name::new("Ground"),
    ));
}
