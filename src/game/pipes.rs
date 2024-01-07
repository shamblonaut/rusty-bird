use bevy::prelude::*;
use rand::Rng;

use crate::game::Scrollable;

use crate::{game::ground::GROUND_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Component)]
pub struct Column;

#[derive(Component)]
pub struct IncomingColumn;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct Gap {
    pub position: f32,
    pub size: f32,
}

#[derive(Event)]
pub struct PipeSpawnEvent;

#[derive(Resource)]
pub struct PipeSpawnTimer(pub Timer);

const PIPE_WIDTH: f32 = 52.0;
const PIPE_HEIGHT: f32 = 320.0;
const GAP_SIZE: f32 = PIPE_HEIGHT / 3.0;

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

pub fn spawn_column(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_event: EventReader<PipeSpawnEvent>,
) {
    for _ in spawn_event.read() {
        let mut rng = rand::thread_rng();
        let gap_position: f32 = (rng.gen_range(
            (((-SCREEN_HEIGHT / 2.0) + GROUND_HEIGHT + (GAP_SIZE * 1.5)) / 50.0) as i32
                ..(((SCREEN_HEIGHT / 2.0) - (GAP_SIZE / 2.0)) / 50.0) as i32,
        ) * 50) as f32;

        let column = commands
            .spawn((
                SpatialBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: (SCREEN_WIDTH / 2.0) + (PIPE_WIDTH / 2.0),
                            y: 0.0,
                            z: 1.0,
                        },
                        scale: Vec3 {
                            x: PIPE_WIDTH,
                            y: 1.0,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                Column,
                Gap {
                    position: gap_position,
                    size: GAP_SIZE,
                },
                Scrollable,
                IncomingColumn,
                Name::new("Column"),
            ))
            .id();

        let top_pipe = commands
            .spawn((
                SpriteBundle {
                    texture: asset_server.load("graphics/pipe-top.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::from((1.0, 1.0))),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: gap_position + (PIPE_HEIGHT / 2.0),
                            z: 0.0,
                        },
                        scale: Vec3 {
                            x: 1.0,
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
                    texture: asset_server.load("graphics/pipe-bottom.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::from((1.0, 1.0))),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: gap_position - GAP_SIZE - (PIPE_HEIGHT / 2.0),
                            z: 0.0,
                        },
                        scale: Vec3 {
                            x: 1.0,
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

pub fn despawn_columns(
    mut commands: Commands,
    transforms: Query<(Entity, &Transform), With<Column>>,
) {
    for (column, transform) in &transforms {
        if transform.translation.x <= (-SCREEN_WIDTH / 2.0) - (PIPE_WIDTH / 2.0) {
            commands.entity(column).despawn_recursive();
        }
    }
}
