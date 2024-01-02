mod faby;
mod ground;
mod pipes;

use crate::{
    game::{
        faby::{check_collision, drop_faby, spawn_faby},
        ground::{reset_ground, spawn_ground},
        pipes::{despawn_columns, spawn_column, spawn_columns},
    },
    AppState,
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
            1.5,
            TimerMode::Repeating,
        )));
        app.insert_resource(FabyDead(false));
        app.add_event::<PipeSpawnEvent>();
        app.add_systems(Startup, spawn_faby);
        app.add_systems(Startup, spawn_ground);

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
pub struct Scrollable;

const SCROLL_SPEED: f32 = 300.0;

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
