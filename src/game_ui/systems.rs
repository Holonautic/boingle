use crate::cards::components::{OnClickOnShopCard, ShopCard, ShopCardType};
use crate::gadgets::resources::GameResources;
use crate::game_ui::components::*;
use crate::gameplay::components::*;
use crate::gameplay::events::OnGadgetCardSelected;
use crate::gameplay::game_states::LevelState;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_bundled_observers::observers;
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::WyRand;
use bevy_simple_subsecond_system::hot;
use bevy_vector_shapes::painter::ShapeCommands;
use num_format::{Locale, ToFormattedString};

#[derive(Component)]
struct DestroyOnHot;

#[derive(Component)]
struct MainUi;

#[hot(rerun_on_hot_patch = true)]
pub fn setup_ui(
    mut commands: Commands,
    player: Single<&Player>,
    destroy_query: Query<Entity, With<MainUi>>,
) {
    info!("Setup main ui UI");
    for entity in destroy_query.iter() {
        commands.entity(entity).despawn();
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
    )>,
) {
    set.p0().0 = format!("{}", player.points);
    set.p1().0 = format!("{}", player.coins);
    set.p2().0 = format!("{}", player.balls_left);
    set.p3().0 = format!("{}", player.point_for_next_level);
    set.p4().0 = format!("{}", player.current_level);
}

const NORMAL_BUTTON: Color = Color::srgb(0.231, 0.51, 0.965);
const HOVERED_BUTTON: Color = Color::srgb(0.145, 0.349, 0.843);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[hot(rerun_on_hot_patch = true)]
pub fn spawn_level_over_ui(
    mut commands: Commands,
    ui_level_over_query: Query<Entity, With<UiLevelOver>>,
) {
    for entity in ui_level_over_query.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn((
        UiLevelOver,
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
) {
    for entity in destroy_query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
struct DestroyShowWidgetSelectionUi;
#[hot(rerun_on_hot_patch = true)]
pub fn show_widget_selection(
    mut commands: Commands,
    shapes: ShapeCommands,
    previous_setup: Query<Entity, With<DestroyOnHot>>,
    gadget_resource: Res<GameResources>,
    mut player: Single<&mut Player>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for entity in previous_setup.iter() {
        commands.entity(entity).despawn();
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
    let last_round_points_formatted = player.points_last_round.to_formatted_string(&Locale::en);

    let font_size = 20.0;

    if player.points_last_round > 0 {
        commands.spawn((
            DestroyShowWidgetSelectionUi,
            UiWidgetSelectionRoot,
            Name::new("widget_selection_ui"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                top: Val::Px(500.0),
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                justify_content: JustifyContent::Center, // center horizontally
                align_items: AlignItems::Center,         // center vertically within bar
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
                        Text::new("Points Last Round:"),
                        TextFont {
                            font_size,
                            ..default()
                        }
                    ),
                    (
                        UiPointsText,
                        Text::new(last_round_points_formatted),
                        TextFont {
                            font_size,
                            ..default()
                        }
                    )
                ]
            ),],
        ));
    }
}

#[hot(rerun_on_hot_patch = true)]
pub fn create_shop_ui(
    mut commands: Commands,
    player: Single<&Player>,
    game_resources: Res<GameResources>,
    mut rng: GlobalEntropy<WyRand>,
    reload_query: Query<Entity, With<UiShopElement>>,
) {
    for entity in reload_query.iter() {
        commands.entity(entity).despawn();
    }
    info!("we are creating shop");

    let cards = game_resources.get_shop_cards_for_level(player.current_level, &mut rng);

    let mut position_x = -300.0;
    let position_z = 50.0;

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
                ShopCard::new(card.clone()),
                OnClickOnShopCard,
                children![(
                    Sprite::from_color(Color::from(tailwind::GRAY_800), Vec2::new(120.0, 25.0)),
                    Transform::from_xyz(0.0, -130.0, 0.0),
                    Text2d(price_text),
                    TextColor(price_color),
                    TextFont {
                        font_size: 10.0,
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
    let font_size = 30.0;
    let font_color = Color::from(tailwind::RED_300);

    commands.spawn((
        UiShopElement,
        Transform::from_xyz(0.0, 140.0, position_z),
        Text2d(format!("Shop Level: {}", player.current_level)),
        TextFont {
            font_size,
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
                next_state.set(LevelState::PlaceWidget);
            }]
        )],
    ));
}
