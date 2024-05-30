use ball::*;
use bevy::prelude::*;
use bot::BotPlugin;
use gutter::*;
use paddle::*;
use score::ScorePlugin;
use scoreboard::ScoreBoardPlugin;

mod ball;
mod bot;
mod collision;
mod gutter;
mod paddle;
mod player;
mod position;
mod score;
mod scoreboard;
mod shape;
mod velocity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScorePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(GutterPlugin)
        .add_plugins(ScoreBoardPlugin)
        .add_plugins(BotPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}
