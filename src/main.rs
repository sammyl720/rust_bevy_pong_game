use ball::*;
use bevy::prelude::*;
use paddle::*;

mod ball;
mod collision;
mod paddle;
mod position;
mod shape;
mod velocity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BallPlugin)
        .add_plugins(PaddlePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}
