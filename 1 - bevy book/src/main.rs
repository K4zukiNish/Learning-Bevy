// cargo run --features bevy/dynamic_linking
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(GreetingTimer(Timer::from_seconds(2.5, TimerMode::Repeating)))
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game".to_string(),
                resolution: (800., 500.).into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    minimize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }), HelloPlugin))
        .add_systems(Startup, spawn_people)
        .add_systems(Update, (update_people, greet_everyone).chain())
        .run();
}

fn spawn_people(mut cmd: Commands) {
    cmd.spawn((Person, Name("Werty".to_string())));
    cmd.spawn((Person, Name("Ytrew".to_string())));
    cmd.spawn((Person, Name("Rew yt".to_string())));
}

fn greet_everyone(time: Res<Time>, mut timer: ResMut<GreetingTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
       for person in &query {
            println!("Hello, {}!", person.0);
        };
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Rew yt" {
            name.0 = "Rue Yr".to_string();
            break; // We don’t need to change any other names
        }
    }
}

#[derive(Resource)]
struct GreetingTimer(Timer);


pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, _app: &mut App) {
        hello_world();
    }
}

pub fn hello_world() {
    println!("Hello world!");
}
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
