use ball::*;
use bevy::prelude::*;

mod ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BallPlugin)
        .run();
}
