use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(hello_world_system)
        .run();
}

fn hello_world_system() {
    println!("Hello world!");
}
