use bevy::prelude::*;

use crate::game::pipes::{Gap, IncomingColumn};
use crate::AppState;

use crate::{game::ground::GROUND_HEIGHT, SCREEN_HEIGHT};

#[derive(Component)]
pub struct Faby;

#[derive(Component)]
pub struct Velocity(f32);

#[derive(Resource)]
pub struct FabyDead(pub bool);

const FALL_SPEED: f32 = 80.0;

pub fn spawn_faby(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 3.0,
                },
                scale: Vec3 {
                    x: 50.0,
                    y: 50.0,
                    z: 1.0,
                },
                ..default()
            },
            ..default()
        },
        Faby,
        Velocity(0.0),
        Name::new("Faby"),
    ));
}

pub fn drop_faby(
    mut transforms: Query<(&mut Transform, &mut Velocity), With<Faby>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut faby_dead: ResMut<FabyDead>,
) {
    for (mut transform, mut velocity) in &mut transforms {
        velocity.0 -= FALL_SPEED;

        if transform.translation.y
            <= (-SCREEN_HEIGHT / 2.0) + (transform.scale.y / 2.0) + GROUND_HEIGHT
        {
            velocity.0 = 0.0;
            next_state.set(AppState::GameOver);
            faby_dead.0 = true;
            return;
        }

        if input.just_pressed(KeyCode::Space) && !faby_dead.0 {
            velocity.0 = 1000.0;
        }

        if transform.translation.y >= (SCREEN_HEIGHT / 2.0) - (transform.scale.y / 2.0) {
            velocity.0 = -FALL_SPEED;
            transform.translation.y = (SCREEN_HEIGHT / 2.0) - (transform.scale.y / 2.0);
        }

        transform.translation.y += velocity.0 * time.delta_seconds();
    }
}

pub fn check_collision(
    mut commands: Commands,
    faby_transform: Query<&Transform, With<Faby>>,
    mut faby_dead: ResMut<FabyDead>,
    column: Query<(Entity, &Transform, &Gap), With<IncomingColumn>>,
) {
    let faby_transform = faby_transform.single();

    for (column_entity, column_transform, gap) in &column {
        if faby_transform.translation.x + (faby_transform.scale.x / 2.0)
            <= column_transform.translation.x - (column_transform.scale.x / 2.0)
        {
            return;
        }

        if (faby_transform.translation.x - (faby_transform.scale.x / 2.0)
            <= column_transform.translation.x + (column_transform.scale.x / 2.0))
            && (faby_transform.translation.y + (faby_transform.scale.y / 2.0) >= gap.position
                || faby_transform.translation.y - (faby_transform.scale.y / 2.0)
                    <= (gap.position - gap.size))
        {
            faby_dead.0 = true;
            return;
        }

        if faby_transform.translation.x - (faby_transform.scale.x / 2.0)
            >= column_transform.translation.x + (column_transform.scale.x / 2.0)
        {
            commands.entity(column_entity).remove::<IncomingColumn>();
        }
    }
}
