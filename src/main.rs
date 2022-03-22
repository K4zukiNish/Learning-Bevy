use bevy::prelude::*;

#[derive(Component)]
struct Character {
    name: String,
}

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_startup_system(startup_game)
        .add_system(msg)
        .run();
}

fn startup_game(mut commands: Commands) {
    commands.spawn().insert(Character {
        name: "Daniel".to_string(),
    });
    commands.spawn().insert(Character {
        name: "Yuliya".to_string(),
    });
    commands.spawn().insert(Character {
        name: "Pozina".to_string(),
    });
}
fn msg(query: Query<&Character>) {
    for character in query.iter() {
        println!("the {} exists", character.name);
    }
}
