use bevy::prelude::*;

use crate::game::Scrollable;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Component)]
pub struct GroundsContainer;

#[derive(Component)]
pub struct Ground;

pub const GROUND_HEIGHT: f32 = SCREEN_HEIGHT / 5.0;

pub fn spawn_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    let container = commands
        .spawn((
            SpatialBundle::default(),
            GroundsContainer,
            Name::new("Grounds Container"),
        ))
        .id();

    let texture = asset_server.load("graphics/ground.png");

    let ground_1 = commands
        .spawn((
            SpriteBundle {
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::from((1.0, 1.0))),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: (-SCREEN_HEIGHT / 2.0) + (GROUND_HEIGHT / 2.0),
                        z: 2.0,
                    },
                    scale: Vec3::from((SCREEN_WIDTH, GROUND_HEIGHT, 1.0)),
                    ..default()
                },
                ..default()
            },
            Ground,
            Scrollable,
            Name::new("Ground 1"),
        ))
        .id();
    commands.entity(container).push_children(&[ground_1]);

    let ground_2 = commands
        .spawn((
            SpriteBundle {
                texture: texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::from((1.0, 1.0))),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: SCREEN_WIDTH,
                        y: (-SCREEN_HEIGHT / 2.0) + (GROUND_HEIGHT / 2.0),
                        z: 2.0,
                    },
                    scale: Vec3::from((SCREEN_WIDTH, GROUND_HEIGHT, 1.0)),
                    ..default()
                },
                ..default()
            },
            Ground,
            Scrollable,
            Name::new("Ground 2"),
        ))
        .id();
    commands.entity(container).push_children(&[ground_2]);
}

pub fn reset_ground(mut transforms: Query<&mut Transform, With<Ground>>) {
    for mut transform in &mut transforms {
        if transform.translation.x < -SCREEN_WIDTH {
            transform.translation.x += SCREEN_WIDTH * 2.0;
        }
    }
}
