#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
struct Character {
    name: String,
    age: u32,
}

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
fn msg(query: Query<&Character>) {
    for character in query.iter() {
        println!("the {} exists", character.name);
    }
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_game).add_system(msg);
    }
}

fn main() {
    App::new().add_plugin(HelloPlugin).run();
}
