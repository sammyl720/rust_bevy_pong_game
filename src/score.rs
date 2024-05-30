use bevy::prelude::*;

use crate::{position::Position, velocity::Velocity, Ball, BALL_SPEED};

enum Scorer {
    Bot,
    Player,
}

#[derive(Event)]
pub struct Scored(Scorer);

#[derive(Resource, Default)]
pub struct Score {
    player: u32,
    bot: u32,
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_event::<Scored>()
            .add_systems(Update, (detect_scoring, reset_ball, update_score).chain());
    }
}

fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.get_single_mut() {
            if ball.0.x > window_width / 2. {
                events.send(Scored(Scorer::Bot));
            } else if ball.0.x < -window_width / 2. {
                events.send(Scored(Scorer::Player));
            }
        }
    }
}

fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((mut position, mut velocity)) = ball.get_single_mut() {
            match event.0 {
                Scorer::Bot => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(-BALL_SPEED, BALL_SPEED);
                }
                Scorer::Player => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(BALL_SPEED, BALL_SPEED);
                }
            }
        }
    }
}

fn update_score(mut score: ResMut<Score>, mut events: EventReader<Scored>) {
    for event in events.read() {
        match event.0 {
            Scorer::Bot => score.bot += 1,
            Scorer::Player => score.player += 1,
        }
    }

    println!("Score: {} - {}", score.player, score.bot);
}
