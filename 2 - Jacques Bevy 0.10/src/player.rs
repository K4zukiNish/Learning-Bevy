use bevy::{math::f32, prelude::*, window::PrimaryWindow};

use crate::clip;

#[derive(Component)]
pub struct Player;

const VELOCITY: f32 = 120.;
const DIRECTINALS: [KeyCode; 8] = [
    KeyCode::KeyA,
    KeyCode::KeyD,
    KeyCode::KeyS,
    KeyCode::KeyW,
    KeyCode::ArrowDown,
    KeyCode::ArrowUp,
    KeyCode::ArrowLeft,
    KeyCode::ArrowRight,
];

pub fn spawn_player(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        Player,
        SpriteBundle {
            texture: assets.load("sprites/ball_blue_small_alt.png"),
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            ..default()
        },
    ));
}

pub fn move_player(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut players_transform: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = players_transform.get_single_mut() {
        if input.any_pressed(DIRECTINALS) {
            let mut direction = Vec3::ZERO;
            for key in input.get_pressed() {
                direction += match key {
                    KeyCode::KeyA | KeyCode::ArrowLeft => Vec3::new(-1., 0., 0.),
                    KeyCode::KeyD | KeyCode::ArrowRight => Vec3::new(1., 0., 0.),
                    KeyCode::KeyS | KeyCode::ArrowDown => Vec3::new(0., -1., 0.),
                    KeyCode::KeyW | KeyCode::ArrowUp => Vec3::new(0., 1., 0.),
                    _ => Vec3::ZERO,
                }
            }
            let speed = if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
                VELOCITY * 2.5
            } else {
                VELOCITY
            };
            transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();
        }
    }
}

pub fn confine_player(
    mut players_transform: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = players_transform.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let width_limit = window.width() / 2.;
        let height_limit = window.height() / 2.;

        transform.translation.x = clip(transform.translation.x, -width_limit, width_limit);
        transform.translation.y = clip(transform.translation.y, -height_limit, height_limit);
    }
}
