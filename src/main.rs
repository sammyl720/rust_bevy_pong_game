use ball::*;
use bevy::prelude::*;
use paddle::*;

mod ball;
mod paddle;
mod position;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BallPlugin)
        .add_plugins(PaddlePlugin)
        .run();
}
