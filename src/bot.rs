use bevy::prelude::*;
use rand::Rng;

use crate::{position::Position, velocity::Velocity, Ball, PADDLE_SPEED};

#[derive(Component)]
pub struct Bot;

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, move_bot);
    }
}

fn move_bot(
    mut bot: Query<(&mut Velocity, &Position), With<Bot>>,
    ball: Query<&Position, With<Ball>>,
) {
    if let Ok((mut velocity, position)) = bot.get_single_mut() {
        if let Ok(ball_position) = ball.get_single() {
            let a_to_b = ball_position.0 - position.0;
            let speed_factor = if random_bool() { 2.5 } else { 3. };

            velocity.0.y = a_to_b.y.signum() * (PADDLE_SPEED * speed_factor);
        }
    }
}

fn random_bool() -> bool {
    rand::thread_rng().gen_bool(0.5)
}
