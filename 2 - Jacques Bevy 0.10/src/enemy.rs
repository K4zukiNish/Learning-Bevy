use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::*;

const VELOCITY: f32 = 150.;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec3,
}

pub fn spawn_enemies(
    mut cmd: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let position = draw_random_vec(
        [-window.width() + 150., window.width() - 150.],
        [-window.height() + 150., window.height() - 150.],
    );
    for _ in 0..ENEMIES_LIMIT {
        cmd.spawn((
            Enemy {
                direction: draw_random_vec([-120., 120.], [-120., 120.]).normalize(),
            },
            SpriteBundle {
                texture: assets.load("sprites/ball_red_small.png"),
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..default()
            },
        ));
    }
}

pub fn move_enemies(mut enemies_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemies_query.iter_mut() {
        transform.translation += enemy.direction * time.delta_seconds() * VELOCITY;
    }
}

pub fn confine_enemies(
    mut enemies_transform: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width_limit = window.width() / 2.;
    let height_limit = window.height() / 2.;

    for (mut transform, mut enemy) in enemies_transform.iter_mut() {
        if transform.translation.x >= width_limit || transform.translation.x <= -width_limit {
            enemy.direction.x *= -1.;
        }
        if transform.translation.y >= height_limit || transform.translation.y <= -height_limit {
            enemy.direction.y *= -1.;
        }
        transform.translation.x = clip(transform.translation.x, -width_limit, width_limit);
        transform.translation.y = clip(transform.translation.y, -height_limit, height_limit);
    }
}

fn draw_random_vec(x: [f32; 2], y: [f32; 2]) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen_range(x[0]..x[1]), rng.gen_range(y[0]..y[1]), 0.)
}
