use crate::game_ui::components::*;
use crate::gameplay::components::Player;
use crate::gameplay::game_states::LevelState;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_bundled_observers::observers;
use bevy_simple_subsecond_system::hot;

#[derive(Component)]
struct DestroyOnHot;

#[hot(rerun_on_hot_patch = true)]
pub fn setup_ui(mut commands: Commands, destroy_query: Query<Entity, With<DestroyOnHot>>) {
    for entity in destroy_query.iter() {
        commands.entity(entity).despawn();
    }

    let font_size = 20.0;
    commands.spawn((
        DestroyOnHot,
        Name::new("ui_root"),
        Node {
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            position_type: PositionType::Absolute,
            // height: Val::Percent(100.0),
            // width: Val::Percent(100.0),
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
        DestroyOnHot,
        Name::new("ui_top_right"),
        Node {
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        children![
        (
            Text::new("Balls:"),
            TextFont {
                font_size,
                ..default()
            }
        ),
        (
            UiBallsText,
            Text::new("0"),
            TextFont {
                font_size,
                ..default()
            }
        )
    ],
    ));
}

#[hot]
pub fn update_ui(
    player: Single<&Player>,
    mut points_text: Single<&mut Text, (With<UiPointsText>, Without<UiCoinsText>,  Without<UiBallsText>)>,
    mut gold_text: Single<&mut Text, (With<UiCoinsText>, Without<UiPointsText>, Without<UiBallsText>)>,
    mut balls_text: Single<&mut Text, (With<UiBallsText>,Without<UiCoinsText>, Without<UiPointsText>)>,

) {
    points_text.0 = format!("{}", player.points);
    gold_text.0 = format!("{}", player.coins);
    balls_text.0 = format!("{}", player.balls_left);
}

const NORMAL_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
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
                TextShadow::default(),
            )],
            observers![
                (|_: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  ui_level_over_query: Query<Entity, With<UiLevelOver>>,
                  mut next_state: ResMut<NextState<LevelState>>| {
                    for entity in ui_level_over_query.iter() {
                        commands.entity(entity).despawn();
                    }
                    next_state.set(LevelState::LevelStart);
                })
            ]
        )],
    ));
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
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
