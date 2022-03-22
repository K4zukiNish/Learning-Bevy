use bevy::prelude::*;

fn main() {
    App::new()
        .add_system(msg)
        .run();
}

fn msg() {
    println!("test with Bevy");
}
