use super::super::constants::*;
use super::super::AppState;
use super::components::PlayerScoreBoard;
use super::components::*;
use super::constants::*;
use super::events::{AudioEvent, ScoreEvent};
use super::resources::Scoreboard;

//use super::ship::yard::default_enemy_ship_bundle;
use bevy::{prelude::*, time::*};

pub struct UiPlugin;

// Plugin definition
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 })
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_scoreboard))
            .add_event::<ScoreEvent>()
            .add_event::<AudioEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(on_score_event),
            );
    }
}

fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "SCORE: ",
                    TextStyle {
                        font: asset_server.load("fonts/Arame-Bold.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: UI_COLOR,
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/Arame-Bold.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: UI_COLOR,
                    },
                ),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(SCOREBOARD_TEXT_PADDING),
                    left: Val::Px(SCREEN_WIDTH * 0.10),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(PlayerScoreBoard);
}

fn on_score_event(
    mut score_events: EventReader<ScoreEvent>,
    mut scoreboard: ResMut<Scoreboard>,
    mut text_query: Query<&mut Text, With<PlayerScoreBoard>>,
) {
    for score_event in score_events.iter() {
        scoreboard.score += score_event.increment;
        let mut player_score_text = text_query.single_mut();
        player_score_text.sections[1].value = scoreboard.score.to_string();
    }
}
