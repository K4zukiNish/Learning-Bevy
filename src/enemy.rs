use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

const NUM_ENEMIES: usize = 5;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy);
    }
}

#[derive(Component)]
pub struct Enemy;

fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_ENEMIES {
        let position = (
            random::<f32>() * window.width(),
            random::<f32>() * window.height(),
        );

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(position.0, position.1, 0.0),
                texture: assets.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy,
        ));
    }
}
