use crate::cards::components::{OnClickOnShopCard, ShopCard};
use crate::gadgets::resources::GameResources;
use crate::game_ui::components::*;
use crate::gameplay::components::*;
use crate::gameplay::events::OnGadgetCardSelected;
use crate::gameplay::game_states::{AppState, LevelState, MenuState};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_bundled_observers::bevy_ecs::spawn::SpawnIter;
use bevy_bundled_observers::observers;
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::WyRand;
use bevy_simple_subsecond_system::hot;
use rand::prelude::*;

#[derive(Component)]
struct DestroyOnHot;

#[derive(Component)]
pub struct MainUi;

#[hot(rerun_on_hot_patch = true)]
pub fn setup_ui(
    mut commands: Commands,
    player: Single<&Player>,
    destroy_query: Query<Entity, With<MainUi>>,
) {
    info!("Setup main ui UI");
    for entity in destroy_query.iter() {
        commands.entity(entity).try_despawn();
    }

    let font_size = 20.0;
    commands.spawn((
        MainUi,
        Name::new("ui_root"),
        Node {
            top: Val::Px(5.0),
            left: Val::Px(10.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        },
        children![
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(4.0),
                    ..default()
                },
                children![
                    (
                        Text::new("Points:"),
                        TextFont {
                            font_size,
                            ..default()
                        }
                    ),
                    (
                        UiPointsText,
                        Text::new("0"),
                        TextFont {
                            font_size,
                            ..default()
                        }
                    )
                ]
            ),
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(4.0),
                    ..default()
                },
                children![
                    (
                        Text::new("Last Round Points:"),
                        TextFont {
                            font_size: font_size * 0.75,
                            ..default()
                        }
                    ),
                    (
                        UiLastRoundPointsText,
                        Text::new("0"),
                        TextFont {
                            font_size: font_size * 0.75,
                            ..default()
                        }
                    )
                ]
            ),
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(4.0),
                height: Val::Px(5.0),
                ..default()
            },
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(4.0),
                    ..default()
                },
                children![
                    (
                        Text::new("Coins:"),
                        TextColor(tailwind::YELLOW_400.into()),
                        TextFont {
                            font_size,
                            ..default()
                        }
                    ),
                    (
                        UiCoinsText,
                        Text::new("0"),
                        TextColor(tailwind::YELLOW_400.into()),
                        TextFont {
                            font_size,
                            ..default()
                        }
                    )
                ]
            )
        ],
    ));

    commands.spawn((
        MainUi,
        Name::new("next_level_goal_ui"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(100.0),
            top: Val::Px(10.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            justify_content: JustifyContent::Center, // center horizontally
            align_items: AlignItems::FlexStart,      // center vertically within bar
            flex_direction: FlexDirection::Row,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(4.0),
                ..default()
            },
            children![
                (
                    Text::new("Goal:"),
                    TextFont {
                        font_size: font_size * 1.3,
                        ..default()
                    },
                    TextColor(tailwind::CYAN_300.into()),
                ),
                (
                    UiPointsForNextLevel,
                    Text::new(format!("{}", player.point_for_next_level)),
                    TextFont {
                        font_size: font_size * 1.3,
                        ..default()
                    },
                    TextColor(tailwind::CYAN_300.into()),
                )
            ]
        ),],
    ));

    commands.spawn((
        MainUi,
        Name::new("ui_top_right"),
        Node {
            top: Val::Px(5.0),
            right: Val::Px(10.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(4.0),
                    ..default()
                },
                children![
                    (
                        Text::new("Level:"),
                        TextFont {
                            font_size,
                            ..default()
                        },
                        TextColor(tailwind::GRAY_400.into()),
                    ),
                    (
                        UiCurrentLevelText,
                        Text::new("0"),
                        TextFont {
                            font_size,
                            ..default()
                        },
                        TextColor(tailwind::GRAY_400.into()),
                    )
                ]
            ),
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(4.0),
                    ..default()
                },
                children![
                    (
                        Text::new("Balls:"),
                        TextFont {
                            font_size,
                            ..default()
                        },
                        TextColor(tailwind::RED_300.into()),
                    ),
                    (
                        UiBallsText,
                        Text::new("0"),
                        TextFont {
                            font_size,
                            ..default()
                        },
                        TextColor(tailwind::RED_300.into()),
                    )
                ]
            )
        ],
    ));
}

#[hot]
pub fn update_ui(
    player: Single<&Player>,
    mut set: ParamSet<(
        Single<&mut Text, With<UiPointsText>>,
        Single<&mut Text, With<UiCoinsText>>,
        Single<&mut Text, With<UiBallsText>>,
        Single<&mut Text, With<UiPointsForNextLevel>>,
        Single<&mut Text, With<UiCurrentLevelText>>,
        Single<&mut Text, With<UiLastRoundPointsText>>,
    )>,
) {
    set.p0().0 = format!("{}", player.points);
    set.p1().0 = format!("{}", player.coins);
    set.p2().0 = format!("{}", player.balls_left);
    set.p3().0 = format!("{}", player.point_for_next_level);
    set.p4().0 = format!("{}", player.current_level);
    set.p5().0 = format!("{}", player.points_last_round);
}

#[hot(rerun_on_hot_patch = true)]
pub fn spawn_level_over_ui(
    mut commands: Commands,
    ui_level_over_query: Query<Entity, With<UiLevelOver>>,
) {
    for entity in ui_level_over_query.iter() {
        commands.entity(entity).try_despawn();
    }
    commands.spawn((
        UiLevelOver,
        Pickable::IGNORE,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Retry"),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow {
                    offset: Vec2::new(2.0, 2.0),
                    ..default()
                },
            )],
            observers![|_: Trigger<Pointer<Click>>,
                        mut commands: Commands,
                        ui_level_over_query: Query<Entity, With<UiLevelOver>>,
                        mut next_state: ResMut<NextState<LevelState>>| {
                for entity in ui_level_over_query.iter() {
                    commands.entity(entity).despawn();
                }
                next_state.set(LevelState::LevelStart);
            }]
        )],
    ));
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = tailwind::RED_500.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

#[derive(Component)]
pub(super) struct DestroyOnWidgetReload;

pub fn widget_selection_ui_despawn(
    _: Trigger<OnGadgetCardSelected>,
    mut commands: Commands,
    destroy_query: Query<Entity, With<UiWidgetSelectionRoot>>,
    previous_setup: Query<Entity, With<DestroyShowWidgetSelectionUi>>,
) {
    for entity in previous_setup.iter() {
        commands.entity(entity).try_despawn();
    }
    for entity in destroy_query.iter() {
        commands.entity(entity).try_despawn();
    }
}

#[derive(Component)]
pub struct DestroyShowWidgetSelectionUi;
#[hot(rerun_on_hot_patch = true)]
pub fn show_widget_selection(
    mut commands: Commands,
    previous_setup: Query<Entity, With<DestroyShowWidgetSelectionUi>>,
    mut player: Single<&mut Player>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for entity in previous_setup.iter() {
        commands.entity(entity).try_despawn();
    }
    let z_position = 50.0;

    while player.current_hand.len() < 3 {
        let next_card = player.next_card(&mut rng);
        player.current_hand.push(next_card);
    }

    let mut x_position = -300.0;

    for card in player.current_hand.iter() {
        commands.spawn((
            ShopCard::new(card.clone()),
            DestroyShowWidgetSelectionUi,
            Transform::from_xyz(x_position, 0.0, z_position),
            OnGadgetCardSelected::new(card.clone()),
        ));
        x_position += 300.0;
    }

    let font_size = 20.0;

    commands.spawn((
        DestroyShowWidgetSelectionUi,
        Transform::from_xyz(0.0, 0.0, 30.0),
        Sprite::from_color(Color::BLACK.with_alpha(0.75), Vec2::new(5000.0, 5000.0)),
    ));

    let root_entity = commands
        .spawn((
            DestroyShowWidgetSelectionUi,
            UiWidgetSelectionRoot,
            Name::new("widget_selection_ui"),
            Pickable::IGNORE,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                justify_content: JustifyContent::Center, // center horizontally
                align_items: AlignItems::Center,         // center vertically within bar
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![(
                Node {
                    top: Val::Px(-135.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(4.0),
                    ..default()
                },
                children![(
                    Text::new("Select a Gadget to Place"),
                    TextFont {
                        font_size,
                        ..default()
                    }
                ),],
            ),],
        ))
        .id();

    let tips = vec![
        "You can use the mouse wheel to rotate some gadgets",
        "You can use the 'R' Key top rotate some gadgets",
        "The shop will always offer you a few free balls",
        "You can always destroy the balls by clicking on the cannon",
    ];

    let tip = tips.choose(&mut rng).unwrap();
    commands.spawn((
        ChildOf(root_entity),
        Node {
            top: Val::Px(160.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(4.0),
            ..default()
        },
        children![(
            Text::new(format!("Tip: {}", tip)),
            TextFont {
                font_size,
                ..default()
            }
        ),],
    ));
}

#[hot(rerun_on_hot_patch = true)]
pub fn show_shop_ui(
    mut commands: Commands,
    player: Single<&Player>,
    game_resources: Res<GameResources>,
    mut rng: GlobalEntropy<WyRand>,
    reload_query: Query<Entity, With<UiShopElement>>,
) {
    for entity in reload_query.iter() {
        commands.entity(entity).try_despawn();
    }

    let cards = game_resources.get_shop_cards_for_level(player.current_level, &mut rng);

    let mut position_x = -300.0;
    let position_z = 50.0;

    commands.spawn((
        UiShopElement,
        Transform::from_xyz(0.0, 0.0, 30.0),
        Sprite::from_color(Color::BLACK.with_alpha(0.95), Vec2::new(5000.0, 5000.0)),
    ));

    for card in cards.iter() {
        let price = game_resources.get_price_per_card(card);
        let price_color = if price > player.coins {
            Color::from(tailwind::RED_700)
        } else {
            Color::from(tailwind::GRAY_50)
        };
        let price_text = if price == 0 {
            "Free!".to_string()
        } else {
            format!("Cost: {} Coins", price)
        };
        let card_entity = commands
            .spawn((
                UiShopElement,
                Transform::from_xyz(position_x, 0.0, position_z),
                Visibility::Visible,
                ShopCard::new(card.clone()),
                OnClickOnShopCard,
                children![(
                    UiCardPriceText,
                    Sprite::from_color(Color::from(tailwind::GRAY_800), Vec2::new(135.0, 25.0)),
                    Transform::from_xyz(0.0, -130.0, 0.0),
                    Text2d(price_text),
                    TextColor(price_color),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                )],
            ))
            .id();

        if player.coins < price {
            commands.entity(card_entity).insert(Forbidden);
        }
        position_x += 200.0
    }
    let font_color = Color::from(tailwind::RED_300);

    commands.spawn((
        UiShopElement,
        Transform::from_xyz(0.0, 240.0, position_z),
        children![
            (
                Text2d("Congrats".to_string()),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(tailwind::YELLOW_500.into())
            ),
            (
                Transform::from_xyz(0.0, -40.0, 0.0),
                Text2d(format!("You Reached Level: {}", player.current_level + 1)),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(tailwind::YELLOW_500.into())
            )
        ],
    ));

    commands.spawn((
        UiShopElement,
        Transform::from_xyz(0.0, 140.0, position_z),
        Text2d("Shop".to_string()),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(font_color),
    ));

    commands.spawn((
        UiShopElement,
        Pickable::IGNORE,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                top: Val::Px(240.0),
                width: Val::Px(180.0),
                height: Val::Px(55.0),
                border: UiRect::all(Val::Px(5.0)),
                position_type: PositionType::Relative,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Exit Shop"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow {
                    offset: Vec2::new(2.0, 2.0),
                    ..default()
                },
            )],
            observers![|_: Trigger<Pointer<Click>>,
                        mut commands: Commands,
                        ui_level_over_query: Query<Entity, With<UiShopElement>>,
                        mut next_state: ResMut<NextState<LevelState>>| {
                for entity in ui_level_over_query.iter() {
                    commands.entity(entity).despawn();
                }
                next_state.set(LevelState::WidgetSelection);
            }]
        )],
    ));
}

pub fn update_shop_ui(
    mut commands: Commands,
    mut ui_shop_price_text: Query<(&mut TextColor, &ChildOf), With<UiCardPriceText>>,
    shop_card_query: Query<&ShopCard>,
    player: Single<&Player>,
    game_resources: Res<GameResources>,
) {
    for (mut text_color, parent) in ui_shop_price_text.iter_mut() {
        let card = shop_card_query.get(parent.0).unwrap();
        let price = game_resources.get_price_per_card(&card.card_type);
        let price_color = if price > player.coins {
            Color::from(tailwind::RED_700)
        } else {
            Color::from(tailwind::GRAY_50)
        };

        text_color.0 = price_color;

        if price > player.coins {
            commands.entity(parent.0).try_insert(Forbidden);
        }
    }
}

pub fn on_entering_shoot_ball_state(
    mut commands: Commands,
    click_on_cannon_text: Single<Entity, With<UiClickOnCannonText>>,
) {
    let entity = click_on_cannon_text.into_inner();
    commands.entity(entity).insert(Visibility::Inherited);
}

pub fn on_exit_shoot_ball_state(
    mut commands: Commands,
    click_on_cannon_text: Single<Entity, With<UiClickOnCannonText>>,
) {
    let entity = click_on_cannon_text.into_inner();
    commands.entity(entity).insert(Visibility::Hidden);
}

#[hot]
pub fn show_menu_ui(mut commands: Commands, previous_query: Query<Entity, With<UiMainMenu>>) {
    for entity in previous_query.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn((
        UiMainMenu,
        Pickable::IGNORE,
        StateScoped(MenuState::MainMenu),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            column_gap: Val::Px(20.0),
            ..default()
        },
        children![
            (header("Boingle"), TextShadow::default(),),
            (
                Node {
                    width: Val::Px(300.0),
                    ..default()
                },
                Text(
                    "Build chaotic pinball contraptions \
                where every ball sparks a chain reaction \
                of bounces, points and coins!"
                        .to_string()
                ),
                TextLayout::new_with_justify(JustifyText::Justified),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
            ),
            (
                button_bundle("Start Game"),
                observers![
                    |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::InGame);
                    }
                ]
            ),
            (
                button_bundle("Credits"),
                observers![
                    |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<MenuState>>| {
                        next_state.set(MenuState::CreditsMenu);
                    }
                ]
            ),
        ],
    ));
}

pub fn show_credits(mut commands: Commands) {
    commands.spawn((
        Pickable::IGNORE,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            column_gap: Val::Px(20.0),
            ..default()
        },
        GlobalZIndex(2),
        StateScoped(MenuState::CreditsMenu),
        children![
            header("Created by"),
            created_by(),
            header("Assets"),
            assets(),
            (
                button_bundle("Back"),
                observers![
                    |_: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<MenuState>>| {
                        next_state.set(MenuState::MainMenu)
                    }
                ]
            )
        ],
    ));
}

pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(40.0),
    )
}
fn assets() -> impl Bundle {
    grid(vec![[
        "Bevy logo",
        "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
    ]])
}

fn created_by() -> impl Bundle {
    grid(vec![
        ["Phil", "Game Idea, Inspiration, Game Design"],
        ["Roger", "Developer, Game Design"],
        ["Emma", "Art"],
        [
            "Bevy Community",
            "Creating amazing engine and help solving all issues",
        ],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Val::Px(10.0),
            column_gap: Val::Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    Text(text.to_string()),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

#[hot]
fn button_bundle(title: impl Into<String>) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(NORMAL_BUTTON),
        children![
            Text::new(title),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow {
                offset: Vec2::new(2.0, 2.0),
                ..default()
            },
        ],
    )
}
