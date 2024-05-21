// cargo run --features bevy/dynamic_linking

use bevy::{prelude::*, window::WindowResolution};

mod enemy;
mod player;

const ENEMIES_LIMIT: u8 = 5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game".to_string(),
                position: WindowPosition::At(IVec2::ZERO),
                resolution: WindowResolution::new(800., 500.),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (player::spawn_player, spawn_camera))
        .add_systems(
            Update,
            (player::move_player, player::confine_player).chain(),
        )
        .add_systems(Startup, enemy::spawn_enemies)
        .add_systems(
            Update,
            (enemy::move_enemies, enemy::confine_enemies).chain(),
        )
        .run();
}

fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

fn clip<T>(value: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
