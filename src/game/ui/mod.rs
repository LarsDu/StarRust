use super::super::constants::*;
use super::super::utils::despawn_all;
use super::super::AppState;
use super::components::PlayerScoreBoard;
use super::constants::*;
use super::events::{AudioEvent, ScoreEvent};
use super::resources::Scoreboard;
use bevy::prelude::*;

pub struct UiPlugin;

// Plugin definition
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 })
            .add_event::<ScoreEvent>()
            .add_event::<AudioEvent>()
            .add_systems(OnEnter(AppState::InGame), setup_scoreboard)
            .add_systems(OnExit(AppState::InGame), despawn_all::<PlayerScoreBoard>)
            .add_systems(Update, on_score_event);
    }
}

fn setup_scoreboard(
    mut scoreboard: ResMut<Scoreboard>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    scoreboard.score = 0;
    commands
        .spawn((
            Text::new("SCORE: 0"),
            TextFont {
                font: asset_server.load("fonts/Arame-Bold.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(UI_COLOR),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(SCOREBOARD_TEXT_PADDING),
                left: Val::Px(SCREEN_WIDTH * 0.10),
                ..default()
            },
        ))
        .insert(PlayerScoreBoard);
}

fn on_score_event(
    mut score_events: EventReader<ScoreEvent>,
    mut scoreboard: ResMut<Scoreboard>,
    mut text_query: Query<&mut Text, With<PlayerScoreBoard>>,
) {
    for score_event in score_events.read() {
        scoreboard.score += score_event.increment;
        if let Ok(mut player_score_text) = text_query.single_mut() {
            **player_score_text = format!("SCORE: {}", scoreboard.score);
        }
    }
    score_events.clear(); // Clear buffer to prevent double registration of scoring events (???)
}
