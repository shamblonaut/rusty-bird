mod faby;
mod ground;
mod pipes;

use crate::{
    game::{
        faby::{check_collision, drop_faby, spawn_faby},
        ground::{reset_ground, spawn_ground},
        pipes::{despawn_columns, spawn_column, spawn_columns},
    },
    AppState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use self::{
    faby::FabyDead,
    pipes::{PipeSpawnEvent, PipeSpawnTimer},
};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeSpawnTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )));
        app.insert_resource(FabyDead(false));
        app.add_event::<PipeSpawnEvent>();
        app.add_systems(Startup, (spawn_background, spawn_faby, spawn_ground));

        app.add_systems(
            Update,
            (
                spawn_columns,
                spawn_column,
                despawn_columns,
                reset_ground,
                scroll_screen,
                drop_faby,
                check_collision,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Background;

fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("graphics/bg-day.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::from((1.0, 1.0))),
                ..default()
            },
            transform: Transform {
                scale: Vec3::from((SCREEN_WIDTH, SCREEN_HEIGHT, 1.0)),
                ..default()
            },
            ..default()
        },
        Background,
        Name::new("Background"),
    ));
}

#[derive(Component)]
pub struct Scrollable;

const SCROLL_SPEED: f32 = 100.0;

fn scroll_screen(
    mut transforms: Query<&mut Transform, With<Scrollable>>,
    time: Res<Time>,
    faby_dead: Res<FabyDead>,
) {
    for mut transform in &mut transforms {
        if faby_dead.0 {
            return;
        }
        transform.translation.x -= SCROLL_SPEED * time.delta_seconds();
    }
}
