use crate::gadgets::components::*;
use crate::gadgets::resources::GameResources;
use crate::gameplay::components::*;
use crate::gameplay::events::{OnCoinCollected, OnGadgetCardSelected};
use crate::gameplay::game_states::LevelState;
use crate::general::components::*;
use crate::general::resources::GameCursor;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingType};
use bevy_rand::prelude::*;
use bevy_simple_subsecond_system::hot;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;

pub fn basic_setup(mut commands: Commands, mut rng: GlobalEntropy<WyRand>) {
    commands.spawn((Name::new("Player"), Player::new(5, &mut rng)));
}

#[hot]
pub fn widget_placement_system(
    mut commands: Commands,
    game_cursor: Res<GameCursor>,
    mut mouse_scroll_event: EventReader<MouseWheel>,
    input: Res<ButtonInput<MouseButton>>,
    mut player: Single<&mut Player>,
    mut q_gadget: Query<(Entity, &mut Transform, &SpriteVisual, &Collider)>,
    mut sprite_query: Query<&mut Sprite>,
    spatial_query: SpatialQuery,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    let Some(current_widget) = player.current_widget else {
        return;
    };

    let Ok((widget_entity, mut widget_transform, sprite_visual, collider)) =
        q_gadget.get_mut(current_widget)
    else {
        return;
    };
    let mut is_intersecting = false;

    for intersection in spatial_query.shape_intersections(
        collider,
        widget_transform.translation.xy(),
        widget_transform.rotation.to_euler(EulerRot::XYZ).2,
        &SpatialQueryFilter::default(),
    ) {
        if intersection != current_widget {
            is_intersecting = true;
        }
    }

    for scroll_event in mouse_scroll_event.read() {
        widget_transform.rotate_z(TAU / 180.0 * scroll_event.y);
    }

    let mut sprite = sprite_query.get_mut(**sprite_visual).unwrap();
    if is_intersecting {
        sprite.color = tailwind::RED_500.into();
    } else {
        sprite.color = Color::WHITE;
    }
    widget_transform.translation = game_cursor.position;

    if !is_intersecting && input.just_pressed(MouseButton::Left) {
        commands
            .entity(widget_entity)
            .insert((PlayerPlacedGadget, Pickable::IGNORE))
            .remove::<Preview>();
        sprite.color.set_alpha(1.0);
        player.current_widget = None;
        next_state.set(LevelState::ShootBall);
    }
}

#[hot]
pub fn increase_power_gauge_system(
    time: Res<Time>,
    mut q_spitter: Query<(&mut BallCannon, &Children), Without<IndicatorGauge>>,
    mut q_indicator: Query<&mut Transform, With<IndicatorGauge>>,
) {
    for (mut spitter, children) in q_spitter.iter_mut() {
        if !spitter.is_increasing_power {
            continue;
        }
        spitter.power = spitter
            .max_power
            .min(spitter.power + time.delta_secs() * spitter.gain);
        let mut indicator_transform = q_indicator.get_mut(children[0]).unwrap();
        let power_gauge = spitter.power / spitter.max_power;
        indicator_transform.translation = Vec3::new(0.0, -(1.0 - power_gauge) * 25.0, 0.1);
        indicator_transform.scale = Vec3::new(1.0, power_gauge, 1.0);
    }
}

#[hot]
pub fn remove_all_placed_gadgets_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    q_gadgets: Query<Entity, With<PlayerPlacedGadget>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for entity in q_gadgets.iter() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct DestroyOnHot;
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
    let card_1_id = spawn_widget_card(
        player.current_hand[0],
        &mut commands,
        &shapes,
        &gadget_resource,
    );
    commands
        .entity(card_1_id)
        .insert((DestroyOnHot, Transform::from_xyz(-300.0, 0.0, z_position)));

    let card_2_id = spawn_widget_card(
        player.current_hand[1],
        &mut commands,
        &shapes,
        &gadget_resource,
    );
    commands
        .entity(card_2_id)
        .insert((DestroyOnHot, Transform::from_xyz(0.0, 0.0, z_position)));

    let card_3_id = spawn_widget_card(
        player.current_hand[2],
        &mut commands,
        &shapes,
        &gadget_resource,
    );
    commands
        .entity(card_3_id)
        .insert((DestroyOnHot, Transform::from_xyz(300.0, 0.0, z_position)));
}
#[hot]
pub fn ball_left_play_area_system(
    mut commands: Commands,
    q_balls: Query<(Entity, &Transform, Option<&Sleeping>), With<PlayerBall>>,
    mut player: Single<&mut Player>,
    mut next_state: ResMut<NextState<LevelState>>,
    state: Res<State<LevelState>>,
) {
    for (ball_entity, transform, sleeping) in q_balls.iter() {
        if sleeping.is_some() || transform.translation.y < -600.0 {
            commands.entity(ball_entity).despawn();
        }
    }

    if (matches!(state.get(), LevelState::BallBouncing) && q_balls.iter().count() == 0) {
        if player.balls_left > 0 {
            player.balls_left -= 1;
            next_state.set(LevelState::EndOfRound)
        } else {
            next_state.set(LevelState::LevelOver)
        }
    }
}

#[hot]
pub fn on_gadget_card_selected(
    trigger: Trigger<OnGadgetCardSelected>,
    mut commands: Commands,
    q_cards: Query<Entity, With<GadgetCard>>,
    mut player: Single<&mut Player>,
    gadget_image_resource: Res<GameResources>,
) {
    for entity in q_cards.iter() {
        commands.entity(entity).despawn();
    }
    

    let gadget_entity = trigger
        .gadget_card
        .spawn_widget(&mut commands, &gadget_image_resource);
    commands.entity(gadget_entity).insert(Preview);
    player.current_widget = Some(gadget_entity);
    let index = player.current_hand.iter().position(|card| card == &trigger.gadget_card).unwrap();
    player.current_hand.remove(index);
}

pub fn clamp_max_ball_velocity(mut q_ball: Query<&mut LinearVelocity, With<PlayerBall>>) {
    for mut velocity in q_ball.iter_mut() {
        let max_velocity = 1000.0;
        if velocity.0.length() > max_velocity {
            velocity.0 = velocity.0.normalize() * max_velocity;
        }
    }
}

pub fn on_coin_collected(
    trigger: Trigger<OnCoinCollected>,
    mut commands: Commands,
    mut player: Single<&mut Player>,
) {
    player.coins += 1;
    commands.entity(trigger.target()).despawn();
}

pub fn on_gadget_deactivated_removed(
    trigger: Trigger<OnRemove, GadgetDeactivated>,
    gadget_deactivated_query: Query<&SpriteVisual, With<Gadget>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    let Ok(sprite_entity) = gadget_deactivated_query.get(trigger.target()) else {
        return;
    };
    let Ok(mut sprite) = sprite_query.get_mut(**sprite_entity) else {
        return;
    };
    sprite.color = Color::WHITE;
}
pub fn on_gadget_deactivated_added(
    trigger: Trigger<OnAdd, GadgetDeactivated>,
    gadget_deactivated_query: Query<&SpriteVisual, With<Gadget>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    let Ok(sprite_entity) = gadget_deactivated_query.get(trigger.target()) else {
        return;
    };
    let Ok(mut sprite) = sprite_query.get_mut(**sprite_entity) else {
        return;
    };
    sprite.color = tailwind::GRAY_700.into();
}

pub fn reactivate_gadgets(mut commands: Commands, mut gadgets_query: Query<(Entity, &mut Gadget)>) {
    for (entity, mut gadget) in gadgets_query.iter_mut() {
        gadget.activations_left = gadget.activations_per_round;
        commands.entity(entity).try_remove::<GadgetDeactivated>();
    }
}

pub fn restarting_level(
    mut commands: Commands,
    q_gadgets: Query<Entity, With<PlayerPlacedGadget>>,
    collectible_query: Query<Entity, With<CollectibleType>>,
    mut player: Single<&mut Player>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut rng: GlobalEntropy<WyRand>,
) {
    for entity in q_gadgets.iter() {
        commands.entity(entity).despawn();
    }
    for entity in collectible_query.iter() {
        commands.entity(entity).despawn();
    }

    player.reset(&mut rng);

    next_state.set(LevelState::PlaceWidget);
}

pub fn end_of_round_system(
    mut commands: Commands,
    mut player: Single<&mut Player>,
    mut shrink_at_end_of_round_query: Query<(Entity, &ShrinkAtEndOfRound, &Transform)>,
    mut remaining_rounds_query: Query<(Entity, &mut RemainingRounds)>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    player.points_last_round = player.points;
    player.points = 0;
    next_state.set(LevelState::PlaceWidget);


    for (entity, shrink, transform) in shrink_at_end_of_round_query.iter_mut() {
        commands.entity(entity).insert(transform.ease_to_fn(
            |start| Transform {
                scale: start.scale * (1.0 - **shrink),
                ..*start
            },
            EaseFunction::QuadraticOut,
            EasingType::Once {
                duration: std::time::Duration::from_secs_f32(0.5),
            },
        ));
    }
    

    for (entity, mut remaining_rounds) in remaining_rounds_query.iter_mut() {
        **remaining_rounds -= 1;

        if **remaining_rounds <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
