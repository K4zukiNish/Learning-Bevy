use bevy::prelude::*;

struct MessageTimer(Timer);
fn message(time: Res<Time>, mut timer: ResMut<MessageTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("A message");
    }
}

struct MainPlugin;
impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MessageTimer(Timer::from_seconds(5., true)))
            .add_system(message);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .run();
}
