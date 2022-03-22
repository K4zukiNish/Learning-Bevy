#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
struct Character {
    name: String,
    age: u32,
}
struct GreetTimer(Timer);

fn startup_game(mut commands: Commands) {
    commands.spawn().insert(Character {
        name: "Daniel".to_string(),
        age: 19,
    });
    commands.spawn().insert(Character {
        name: "Yuliya".to_string(),
        age: 22,
    });
    commands.spawn().insert(Character {
        name: "Pozina".to_string(),
        age: 12,
    });
}
fn msg(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Character>) {
    if timer.0.tick(time.delta()).just_finished() {
        for character in query.iter() {
            println!("the {} exists", character.name);
        }
    }
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2., true)))
            .add_startup_system(startup_game)
            .add_system(msg);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
