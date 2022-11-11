// Adapted from https://github.com/bevyengine/bevy/blob/v0.8.1/examples/games/game_menu.rs
use bevy::{app::AppExit, prelude::*};

use crate::{game::scene::SceneAssets, utils::despawn_all};

use super::AppState;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const BOX_COLOR: Color = Color::rgba(0.25, 0.0, 0.0, 0.06);
const LEVEL_END_BOX_COLOR: Color = Color::rgba(0.0, 0.0, 0.8, 1.0);
const PLAYER_DEATH_BOX_COLOR: Color = Color::rgba(0.25, 0.0, 0.0, 1.0);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum MenuState {
    Main,
    LevelEnd,
    PlayerDeath,
    Disabled,
}

#[derive(Component)]
pub struct MenuBackground;
#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnLevelEndScreen;

#[derive(Component)]
pub struct OnPlayerDeathScreen;
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(switch_to_main_menu))
            .add_state(MenuState::Main)
            .add_system_set(
                SystemSet::on_enter(MenuState::Main)
                    .with_system(main_menu_setup)
                    .with_system(load_background_model),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::Main).with_system(despawn_all::<OnMainMenuScreen>),
            )
            .add_system_set(SystemSet::on_enter(MenuState::LevelEnd).with_system(level_end_setup))
            .add_system_set(
                SystemSet::on_exit(MenuState::LevelEnd)
                    .with_system(despawn_all::<OnLevelEndScreen>),
            )
            .add_system_set(
                SystemSet::on_enter(MenuState::PlayerDeath).with_system(player_death_setup),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::PlayerDeath)
                    .with_system(despawn_all::<OnPlayerDeathScreen>),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(menu_action)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Menu).with_system(despawn_all::<MenuBackground>),
            );
    }
}

// State used for the current menu screen

// Tag component used to mark wich setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
    MainMenu,
    Restart,
}
// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn load_background_model(mut commands: Commands, models: Res<SceneAssets>) {
    commands
        .spawn(SceneBundle {
            scene: models.default_enemy.clone_weak(),
            transform: Transform::from_xyz(0.0, 210.0, 20.0)
                .with_scale(Vec3::splat(30.0))
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 20.0, 95.0, 0.0)),
            ..default()
        })
        .insert(MenuBackground);
    /*
    commands.spawn(SceneBundle {
        scene: models.basic_boss.clone_weak(),
        transform: Transform::from_xyz(0.0, 300.0, 20.0)
            .with_scale(Vec3::splat(10.0))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 95.0, 0.0)),
        ..default()
    }).insert(MenuBackground);
    */
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 25000.0,
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MenuBackground);
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::MainMenu => {
                    menu_state.overwrite_set(MenuState::Main).unwrap();
                    //game_state.overwrite_set(AppState::Menu).unwrap();// PANICS for some reason
                }
                MenuButtonAction::Restart => {
                    menu_state.overwrite_set(MenuState::Main).unwrap();
                    //game_state.overwrite_set(AppState::Menu).unwrap();// PANICS for some reason
                }
                MenuButtonAction::Play => {
                    menu_state.overwrite_set(MenuState::Disabled).unwrap();
                    game_state.overwrite_set(AppState::InGame).unwrap();
                }
            }
        }
    }
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Arame-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        size: Size::new(Val::Px(30.0), Val::Auto),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        position: UiRect {
            left: Val::Px(10.0),
            right: Val::Auto,
            top: Val::Auto,
            bottom: Val::Auto,
        },
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BOX_COLOR.into(),
            ..default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .insert(MenuButtonAction::Quit)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/exitRight.png");
                    parent.spawn(ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage(icon),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                });

            parent
                .spawn(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .insert(MenuButtonAction::Play)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/right.png");
                    parent.spawn(ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage(icon),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section("Play", button_text_style.clone()));
                });

            parent.spawn(
                TextBundle::from_section(
                    "Star Rust",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
}

fn level_end_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Arame-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(315.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: LEVEL_END_BOX_COLOR.into(),
            ..default()
        })
        .insert(OnLevelEndScreen)
        .with_children(|parent| {
            // Display the game name
            parent.spawn(
                TextBundle::from_section(
                    "LEVEL END",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );

            // Display
            // - MAIN MENU
            // - RESTART
            parent
                .spawn(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .insert(MenuButtonAction::MainMenu)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "MAIN MENU",
                        button_text_style.clone(),
                    ));
                });
        });
}

fn player_death_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Arame-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(300.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: PLAYER_DEATH_BOX_COLOR.into(),
            ..default()
        })
        .insert(OnPlayerDeathScreen)
        .with_children(|parent| {
            /*
            parent
                .spawn(ButtonBundle {
                    style: button_style,
                    ..default()
                })
                .insert(MenuButtonAction::Restart)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/exitRight.png");
                    parent.spawn(ImageBundle {
                        style: button_icon_style,
                        image: UiImage(icon),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section("RESTART", button_text_style));
                });*/

            parent
                .spawn(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .insert(MenuButtonAction::MainMenu)
                .with_children(|parent| {
                    /*let icon = asset_server.load("textures/Game Icons/right.png");
                    parent.spawn(ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage(icon),
                        ..default()
                    });*/
                    parent.spawn(TextBundle::from_section(
                        "MAIN MENU",
                        button_text_style.clone(),
                    ));
                });

            // Display the game name
            parent.spawn(
                TextBundle::from_section(
                    "GAME OVER",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
}
