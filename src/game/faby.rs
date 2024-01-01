use bevy::prelude::*;

use crate::SCREEN_HEIGHT;

use super::GROUND_HEIGHT;

#[derive(Component)]
pub struct Faby;

#[derive(Component)]
pub struct Velocity(f32);

const FALL_VELOCITY: f32 = 80.0;

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
) {
    for (mut transform, mut velocity) in &mut transforms {
        velocity.0 -= FALL_VELOCITY;

        if transform.translation.y
            <= (-SCREEN_HEIGHT / 2.0) + (transform.scale.y / 2.0) + GROUND_HEIGHT
        {
            velocity.0 = 0.0;
            transform.translation.y =
                (-SCREEN_HEIGHT / 2.0) + (transform.scale.y / 2.0) + GROUND_HEIGHT;
        }

        if input.just_pressed(KeyCode::Space) {
            velocity.0 = 1000.0;
        }

        if transform.translation.y >= (SCREEN_HEIGHT / 2.0) - (transform.scale.y / 2.0) {
            velocity.0 = -FALL_VELOCITY;
            transform.translation.y = (SCREEN_HEIGHT / 2.0) - (transform.scale.y / 2.0);
        }

        transform.translation.y += velocity.0 * time.delta_seconds();
    }
}
