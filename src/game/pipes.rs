use bevy::prelude::*;
use rand::Rng;

use crate::game::PipeSpawnTimer;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::GROUND_HEIGHT;

#[derive(Component)]
pub struct Column;

#[derive(Component)]
pub struct Pipe;

#[derive(Event)]
pub struct PipeSpawnEvent;

const PIPE_WIDTH: f32 = 100.0;
const PIPE_HEIGHT: f32 = SCREEN_HEIGHT - (GROUND_HEIGHT / 2.0);
const GAP_SIZE: f32 = PIPE_HEIGHT / 4.0;

pub fn spawn_columns(
    time: Res<Time>,
    mut pipe_spawn_timer: ResMut<PipeSpawnTimer>,
    mut spawn_event: EventWriter<PipeSpawnEvent>,
) {
    pipe_spawn_timer.0.tick(time.delta());

    if pipe_spawn_timer.0.just_finished() {
        spawn_event.send(PipeSpawnEvent);
    }
}

pub fn spawn_column(mut commands: Commands, mut spawn_event: EventReader<PipeSpawnEvent>) {
    for _ in spawn_event.read() {
        let column = commands
            .spawn((
                SpatialBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: (SCREEN_WIDTH / 2.0) + (PIPE_WIDTH / 2.0),
                            y: 0.0,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                Column,
                Name::new("Column"),
            ))
            .id();

        let gap_position = rand::thread_rng()
            .gen_range(
                ((-SCREEN_HEIGHT / 2.0) + GROUND_HEIGHT + (GAP_SIZE * 1.5)) / 50.0
                    ..((SCREEN_HEIGHT / 2.0) - (GAP_SIZE / 2.0)) / 50.0,
            )
            .trunc()
            * 50.0;
        info!("Gap Position: {}", gap_position);

        let top_pipe = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: gap_position + (PIPE_HEIGHT / 2.0),
                            z: 0.0,
                        },
                        scale: Vec3 {
                            x: PIPE_WIDTH,
                            y: PIPE_HEIGHT,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                Pipe,
                Name::new("Top Pipe"),
            ))
            .id();
        commands.entity(column).push_children(&[top_pipe]);

        let bottom_pipe = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: gap_position - GAP_SIZE - (PIPE_HEIGHT / 2.0),
                            z: 0.0,
                        },
                        scale: Vec3 {
                            x: PIPE_WIDTH,
                            y: PIPE_HEIGHT,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                Pipe,
                Name::new("Top Pipe"),
            ))
            .id();
        commands.entity(column).push_children(&[bottom_pipe]);
    }
}

pub fn move_columns(
    mut commands: Commands,
    mut transforms: Query<(Entity, &mut Transform), With<Column>>,
    time: Res<Time>,
) {
    for (column, mut transform) in &mut transforms {
        transform.translation.x -= 300.0 * time.delta_seconds();

        if transform.translation.x <= (-SCREEN_WIDTH / 2.0) - (PIPE_WIDTH / 2.0) {
            commands.entity(column).despawn_recursive();
        }
    }
}
