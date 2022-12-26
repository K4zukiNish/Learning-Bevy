use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(populate).add_system(hello);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn hello(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello, {}!", name.0);
    }
}

fn populate(mut commands: Commands) {
    commands.spawn_bundle((Person, Name("Elaina Proctor".to_string())));
    commands.spawn_bundle((Person, Name("Renzo Hume".to_string())));
    commands.spawn_bundle((Person, Name("Zayna Nieves".to_string())));
}
