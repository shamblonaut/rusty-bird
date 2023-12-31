use bevy::prelude::*;

use crate::SCREEN_HEIGHT;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_faby);
        app.add_systems(Update, drop_faby);
    }
}

#[derive(Component)]
struct Faby;

#[derive(Component)]
struct Velocity(f32);

fn spawn_faby(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                ..default()
            },
            transform: Transform {
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

fn drop_faby(
    mut transforms: Query<(&mut Transform, &mut Velocity), With<Faby>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    for (mut transform, mut velocity) in &mut transforms {
        velocity.0 -= 100.0;

        if transform.translation.y <= (-SCREEN_HEIGHT / 2.0) + (transform.scale.y / 2.0) {
            velocity.0 = 0.0;
            transform.translation.y = (-SCREEN_HEIGHT / 2.0) + (transform.scale.y / 2.0);
        }

        if input.just_pressed(KeyCode::Space) {
            velocity.0 = 1000.0;
        }

        if transform.translation.y >= (SCREEN_HEIGHT / 2.0) - (transform.scale.y / 2.0) {
            velocity.0 = -100.0;
            transform.translation.y = (SCREEN_HEIGHT / 2.0) - (transform.scale.y / 2.0);
        }

        transform.translation.y += velocity.0 * time.delta_seconds();
    }
}
