use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::*;

#[derive(Component)]
pub struct Enemy;

pub fn spawn_enemies(
    mut cmd: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let position = draw_random_position(
        [-window.width() + 150., window.width() - 150.],
        [-window.height() + 150., window.height() - 150.],
    );
    for _ in 0..ENEMIES_LIMIT {
        cmd.spawn((
            Enemy,
            SpriteBundle {
                texture: assets.load("sprites/ball_red_small.png"),
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..default()
            },
        ));
    }
}

fn draw_random_position(width: [f32; 2], height: [f32; 2]) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(width[0]..width[1]),
        rng.gen_range(height[0]..height[1]),
        0.,
    )
}
