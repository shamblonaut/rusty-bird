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

const FALL_SPEED: f32 = 35.0;

pub fn spawn_faby(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("graphics/faby-red.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::from((1.0, 1.0))),
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 3.0,
                },
                scale: Vec3::from((34.0, 24.0, 1.0)),
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
            velocity.0 = FALL_SPEED * 12.5;
        }

        if transform.translation.y >= (SCREEN_HEIGHT / 2.0) - (transform.scale.y / 2.0) {
            velocity.0 = -FALL_SPEED;
            transform.translation.y = (SCREEN_HEIGHT / 2.0) - (transform.scale.y / 2.0);
        }

        transform.translation.y += velocity.0 * time.delta_seconds();

        if !(transform.rotation.z > 0.3 && velocity.0 > 0.0)
            && !(transform.rotation.z < -0.5 && velocity.0 < 0.0)
        {
            transform.rotate_z(velocity.0 / 35.0 * time.delta_seconds());
        }
        if transform.rotation.z < -0.5 && transform.rotation.z > -0.6 {
            info!("Velocity: {}", velocity.0);
        }
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
