use bevy::{math::vec3, prelude::*};

#[derive(Component)]
pub struct Player;

const VELOCITY: f32 = 120.;

pub fn spawn_player(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        Player,
        SpriteBundle {
            texture: assets.load("sprites/ball_blue_large_alt.png"),
            transform: Transform {
                translation: Vec3::ZERO,
                scale: vec3(0.5, 0.5, 0.5),
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
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::KeyA) {
            direction.x += -1.;
        };
        if input.pressed(KeyCode::KeyS) {
            direction.y += -1.;
        };
        if input.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        };
        if input.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        };

        let speed = match input {
            _ if input.pressed(KeyCode::ShiftLeft) => VELOCITY * 2.5,
            _ => VELOCITY,
        };

        transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();
    }
}
