// Adapted from https://github.com/bevyengine/bevy/blob/v0.8.1/examples/games/game_menu.rs
use bevy::{app::AppExit, prelude::*};

use crate::{game::models::ModelsAssets, utils::despawn_all};

use crate::AppState;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const BOX_COLOR: Color = Color::srgba(0.25, 0.0, 0.0, 0.06);
const LEVEL_END_BOX_COLOR: Color = Color::srgba(0.0, 0.0, 0.8, 1.0);
const PLAYER_DEATH_BOX_COLOR: Color = Color::srgba(0.25, 0.0, 0.0, 1.0);

#[derive(Clone, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
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
        app.init_state::<MenuState>()
            .add_systems(
                OnExit(AppState::Menu),
                (
                    despawn_all::<MenuBackground>,
                    despawn_all::<OnPlayerDeathScreen>,
                ),
            )
            .add_systems(
                OnEnter(MenuState::Main),
                (
                    main_menu_setup,
                    load_background_model,
                    despawn_all::<OnPlayerDeathScreen>,
                ),
            )
            .add_systems(OnExit(MenuState::Main), despawn_all::<OnMainMenuScreen>)
            .add_systems(
                OnEnter(MenuState::LevelEnd),
                (level_end_setup, despawn_all::<OnLevelEndScreen>),
            )
            .add_systems(
                OnEnter(MenuState::PlayerDeath),
                (player_death_setup, despawn_all::<OnPlayerDeathScreen>),
            )
            .add_systems(Update, (menu_action, button_system));
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
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::MainMenu => {
                    menu_state.set(MenuState::Main);
                }
                MenuButtonAction::Restart => {
                    menu_state.set(MenuState::Main);
                }
                MenuButtonAction::Play => {
                    menu_state.set(MenuState::Disabled);
                    game_state.set(AppState::InGame);
                }
            }
        }
    }
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Arame-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        height: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        right: Val::Auto,
        top: Val::Auto,
        bottom: Val::Auto,
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
                    let icon: Handle<Image> =
                        asset_server.load("textures/Game Icons/exitRight.png");
                    parent.spawn(ImageBundle {
                        style: button_icon_style.clone(),
                        image: UiImage {
                            texture: icon,
                            ..default()
                        },
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
                        image: UiImage {
                            texture: icon,
                            ..default()
                        },
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
        width: Val::Px(315.0),
        height: Val::Px(65.0),
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
        width: Val::Px(300.0),
        height: Val::Px(65.0),
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


fn load_background_model(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/basic_enemy.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 210.0, 20.0)
                .with_scale(Vec3::splat(30.0))
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 20.0, 95.0, 0.0)),
            ..default()
        })
        .insert(MenuBackground);
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