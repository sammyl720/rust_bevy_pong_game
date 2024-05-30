use bevy::prelude::*;

use crate::score::Score;

#[derive(Component)]
struct PlayerScoreboard;

#[derive(Component)]
struct BotScoreboard;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scoreboard)
            .add_systems(Update, update_scoreboard);
    }
}

fn update_scoreboard(
    mut player_score: Query<&mut Text, With<PlayerScoreboard>>,
    mut bot_score: Query<&mut Text, (With<BotScoreboard>, Without<PlayerScoreboard>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.sections[0].value = score.player.to_string();
        }

        if let Ok(mut bot_score) = bot_score.get_single_mut() {
            bot_score.sections[0].value = score.bot.to_string();
        }
    }
}

fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a
        // single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts
            // into a `String`, such as `&str`
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        PlayerScoreboard,
    ));

    // Then we do it again for the AI score
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        BotScoreboard,
    ));
}
