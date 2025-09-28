// Adapted from https://github.com/bevyengine/bevy/blob/v0.8.1/examples/games/game_menu.rs
use bevy::{app::AppExit, prelude::*};

use crate::utils::despawn_all;

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
    let button_node = Node {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: Val::Px(30.0),
        height: Val::Px(30.0),
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        right: Val::Auto,
        top: Val::Auto,
        bottom: Val::Auto,
        ..default()
    };
    let button_font = TextFont {
        font: font.clone(),
        font_size: 40.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BOX_COLOR),
        ))
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            parent
                .spawn((Button, button_node.clone(), BackgroundColor(NORMAL_BUTTON)))
                .insert(MenuButtonAction::Quit)
                .with_children(|parent| {
                    let icon: Handle<Image> =
                        asset_server.load("textures/Game Icons/exitRight.png");
                    parent.spawn((button_icon_node.clone(), ImageNode::new(icon)));
                    parent.spawn((
                        Text::new("Quit"),
                        button_font.clone(),
                        TextColor(TEXT_COLOR),
                    ));
                });

            parent
                .spawn((Button, button_node.clone(), BackgroundColor(NORMAL_BUTTON)))
                .insert(MenuButtonAction::Play)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/right.png");
                    parent.spawn((button_icon_node.clone(), ImageNode::new(icon)));
                    parent.spawn((
                        Text::new("Play"),
                        button_font.clone(),
                        TextColor(TEXT_COLOR),
                    ));
                });

            parent.spawn((
                Text::new("Star Rust"),
                TextFont {
                    font: font.clone(),
                    font_size: 80.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            ));
        });
}

fn level_end_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Arame-Bold.ttf");
    let button_node = Node {
        width: Val::Px(315.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_font = TextFont {
        font: font.clone(),
        font_size: 40.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(LEVEL_END_BOX_COLOR),
        ))
        .insert(OnLevelEndScreen)
        .with_children(|parent| {
            // Display the game name
            parent.spawn((
                Text::new("LEVEL END"),
                TextFont {
                    font: font.clone(),
                    font_size: 80.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Display
            // - MAIN MENU
            // - RESTART
            parent
                .spawn((Button, button_node.clone(), BackgroundColor(NORMAL_BUTTON)))
                .insert(MenuButtonAction::MainMenu)
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("MAIN MENU"),
                        button_font.clone(),
                        TextColor(TEXT_COLOR),
                    ));
                });
        });
}

fn player_death_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Arame-Bold.ttf");
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_font = TextFont {
        font: font.clone(),
        font_size: 40.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(PLAYER_DEATH_BOX_COLOR),
        ))
        .insert(OnPlayerDeathScreen)
        .with_children(|parent| {
            /*
            parent
                .spawn((Button, button_node.clone(), BackgroundColor(NORMAL_BUTTON)))
                .insert(MenuButtonAction::Restart)
                .with_children(|parent| {
                    let icon = asset_server.load("textures/Game Icons/exitRight.png");
                    parent.spawn((button_icon_node.clone(), ImageNode::new(icon)));
                    parent.spawn((Text::new("RESTART"), button_font.clone(), TextColor(TEXT_COLOR)));
                });*/

            parent
                .spawn((Button, button_node.clone(), BackgroundColor(NORMAL_BUTTON)))
                .insert(MenuButtonAction::MainMenu)
                .with_children(|parent| {
                    /*let icon = asset_server.load("textures/Game Icons/right.png");
                    parent.spawn((button_icon_node.clone(), ImageNode::new(icon)));*/
                    parent.spawn((
                        Text::new("MAIN MENU"),
                        button_font.clone(),
                        TextColor(TEXT_COLOR),
                    ));
                });

            // Display the game name
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font: font.clone(),
                    font_size: 80.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            ));
        });
}

fn load_background_model(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands
        .spawn((
            SceneRoot(asset_server.load("models/basic_enemy.glb#Scene0")),
            Transform::from_xyz(0.0, 210.0, 20.0)
                .with_scale(Vec3::splat(30.0))
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 20.0, 95.0, 0.0)),
        ))
        .insert(MenuBackground);
    commands
        .spawn((
            DirectionalLight {
                illuminance: 25000.0,
                color: Color::WHITE,
                ..default()
            },
            Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .insert(MenuBackground);
}
