use crate::cards::components::*;
use crate::gadgets::components::CollectibleType::CoinType;
use crate::gadgets::components::*;
use crate::gadgets::resources::GameResources;
use crate::gadgets::systems::on_finish_easing_destroy;
use crate::game_ui::components::Forbidden;
use crate::gameplay::components::*;
use crate::gameplay::events::*;
use crate::gameplay::game_states::LevelState;
use crate::general::components::*;
use crate::general::resources::GameCursor;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_bundled_observers::observers;
use bevy_easings::{Ease, EaseFunction, EasingType};
use bevy_rand::prelude::*;
use bevy_simple_subsecond_system::hot;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;
use std::time::Duration;

pub fn basic_setup(mut commands: Commands, mut rng: GlobalEntropy<WyRand>) {
    commands.spawn((Name::new("Player"), Player::new(5, &mut rng)));
}

// #[hot]
pub fn widget_placement_system(
    mut commands: Commands,
    game_cursor: Res<GameCursor>,
    mut mouse_scroll_event: EventReader<MouseWheel>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Player>,
    mut q_gadget: Query<(Entity, &mut Transform, Option<&SpriteVisual>, &Collider, &CollisionLayers, Has<CanBeRotated>)>,
    mut sprite_query: Query<&mut Sprite>,
    spatial_query: SpatialQuery,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    let Some(current_widget) = player.current_widget else {
        return;
    };

    let Ok((widget_entity, mut widget_transform, sprite_visual, collider, layers, can_be_rotated)) =
        q_gadget.get_mut(current_widget)
    else {
        return;
    };
    let mut is_intersecting = false;



    let filter = SpatialQueryFilter::from_mask(layers.filters);

    for intersection in spatial_query.shape_intersections(
        collider,
        widget_transform.translation.xy(),
        widget_transform.rotation.to_euler(EulerRot::XYZ).2,
        &filter,
    ) {
        if intersection != current_widget {
            is_intersecting = true;
        }
    }

    if can_be_rotated {
        for scroll_event in mouse_scroll_event.read() {
            widget_transform.rotate_z(TAU / 180.0 * scroll_event.y);
        }

        if key_input.pressed(KeyCode::KeyR) {
            widget_transform.rotate_z(TAU / 180.0);
        }

    }


    if let Some(sprite_visual) = sprite_visual {
        let mut sprite = sprite_query.get_mut(**sprite_visual).unwrap();
        if is_intersecting {
            sprite.color = tailwind::RED_500.into();
        } else {
            sprite.color = Color::WHITE;
        }
    }

    widget_transform.translation = game_cursor.position;

    if !is_intersecting && mouse_input.just_pressed(MouseButton::Left) {
        commands
            .entity(widget_entity)
            .insert((PlayerPlacedGadget, Pickable::IGNORE))
            .remove::<Preview>();

        if let Some(sprite_visual) = sprite_visual {
            let mut sprite = sprite_query.get_mut(**sprite_visual).unwrap();
            sprite.color.set_alpha(1.0);
        }
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
        }
    }
}

#[hot]
pub fn on_gadget_card_selected(
    trigger: Trigger<OnGadgetCardSelected>,
    mut commands: Commands,
    shop_cards_query: Query<Entity, With<ShopCard>>,
    mut player: Single<&mut Player>,
    gadget_image_resource: Res<GameResources>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    for entity in shop_cards_query.iter() {
        commands.entity(entity).despawn();
    }

    let gadget_entity = trigger
        .shop_card_type
        .get_gadget_type()
        .unwrap()
        .spawn_widget(&mut commands, &gadget_image_resource);
    commands.entity(gadget_entity).insert(Preview);
    player.current_widget = Some(gadget_entity);
    let index = player
        .current_hand
        .iter()
        .position(|card| card == &trigger.shop_card_type)
        .unwrap();
    let used_card = player.current_hand.remove(index);
    player.discard_pile.push(used_card);

    next_state.set(LevelState::PlaceWidget);
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
    collectible_query: Query<
        Entity,
        Or<(
            With<CollectibleType>,
            With<DecayOverTime>,
            With<PlayerPlacedGadget>,
        )>,
    >,
    cannon_query: Single<(&BallCannon, &Transform)>,
    mut player: Single<&mut Player>,
    mut rng: GlobalEntropy<WyRand>,
    game_resources: Res<GameResources>,
) {
    for entity in collectible_query.iter() {
        commands.entity(entity).try_despawn();
    }
    player.reset(&mut rng);

    commands.trigger(RequestToPlaceCoins::new(5));

    let (cannon, canon_transform) = cannon_query.into_inner();
    let forward = canon_transform.rotation * Vec3::Y;
    let forward_2d = forward.truncate();
    commands.spawn((
        FakePlayerBall,
        Transform::from_translation(canon_transform.translation).with_scale(Vec3::splat(0.5)),
        LinearVelocity(forward_2d * cannon.power),
    ));

    // next_state.set(LevelState::PlaceWidget);
}

pub fn draw_trajectory_system(
    mut commands: Commands,
    trajectory_query: Query<(Entity, &Transform, &DrawTrajectory)>,
    mut shapes: ShapeCommands,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    for (entity, transform, draw_trajectory) in trajectory_query.iter() {
        if transform.translation.y < -800.0 {
            commands.entity(entity).despawn();
            next_state.set(LevelState::WidgetSelection);
        }
        shapes.transform.translation = transform.translation;
        shapes
            .circle(1.0)
            .insert(DecayOverTime::new(draw_trajectory.duration.as_secs_f32()));
    }
}

pub fn end_of_round_system(
    mut commands: Commands,
    mut player: Single<&mut Player>,
    mut shrink_at_end_of_round_query: Query<(Entity, &ShrinkAtEndOfRound, &Transform)>,
    mut remaining_rounds_query: Query<(Entity, &mut RemainingRounds)>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    player.points_last_round = player.points_this_round;
    player.points_this_round = 0;

    info!("We are at the end of round");
    if player.points >= player.point_for_next_level {
        info!("We are going to the shop");
        next_state.set(LevelState::Shop);
    } else if player.balls_left > 0 {
        info!("We are placing a widget");

        next_state.set(LevelState::WidgetSelection);
    } else {
        info!("We are gameover");
        next_state.set(LevelState::GameOver);
    }
    // player.points = 0;

    for (entity, shrink, transform) in shrink_at_end_of_round_query.iter_mut() {
        commands.entity(entity).insert(transform.ease_to_fn(
            |start| Transform {
                scale: start.scale * (1.0 - **shrink),
                ..*start
            },
            EaseFunction::QuadraticOut,
            EasingType::Once {
                duration: Duration::from_secs_f32(0.5),
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

pub fn on_exit_shop(mut commands: Commands, mut player: Single<&mut Player>) {
    player.current_level += 1;
    player.point_for_next_level = Player::points_for_level(player.current_level);
    commands.trigger(RequestToPlaceCoins::new(5));

    info!(
        "We are exiting the shop level: {}, points: {}",
        player.current_level, player.point_for_next_level
    );
}

pub fn destroy_when_standing_still_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &Transform,
        &LinearVelocity,
        &mut DestroyOnStandingStill,
    )>,
) {
    for (entity, transform, still, mut destroy) in query.iter_mut() {
        let Some(last_position) = destroy.last_position else {
            destroy.last_position = Some(transform.translation);
            continue;
        };

        let threshold_squared = destroy.movement_threshold * destroy.movement_threshold;
        if transform.translation.distance_squared(last_position) < threshold_squared {
            destroy.time_since_movement += time.delta();
        }

        if destroy.time_since_movement > destroy.max_time_standing_still {
            commands.entity(entity).despawn();
        }
        destroy.last_position = Some(transform.translation);
    }
}

pub fn on_click_on_shop_card_system(
    trigger: Trigger<OnClickOnShopCard>,
    shop_card_query: Query<(&ShopCard, &Transform), Without<Forbidden>>,
    mut player: Single<&mut Player>,
    mut commands: Commands,
    game_resources: Res<GameResources>,
    mut rng: GlobalEntropy<WyRand>,
) {
    let Ok((card, transform)) = shop_card_query.get(trigger.target()) else {
        return;
    };

    let price = game_resources.get_price_per_card(&card.card_type);

    if price > player.coins {
        return;
    }
    player.coins -= price;

    let transform_ease = transform.ease_to_fn(
        |start| Transform {
            translation: Vec3::new(0.0, -800.0, 100.0),
            ..*start
        },
        EaseFunction::QuadraticInOut,
        EasingType::Once {
            duration: Duration::from_secs_f32(1.0),
        },
    );
    commands.entity(trigger.target()).insert((
        transform_ease,
        Forbidden,
        observers![on_finish_easing_destroy],
    ));
    commands.entity(trigger.target()).remove::<Collider>();

    match card.card_type {
        ShopCardType::OneMoreBallCard => player.balls_left += 1,
        ShopCardType::MoreBallsCard => player.balls_left += game_resources.balls_per_level,
        _ => {
            player.discard_pile.push(card.card_type);
            player.reshuffle_deck(&mut rng);
        }
    }
}

pub fn decay_over_time_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut DecayOverTime, Option<&mut ShapeFill>)>,
    mut commands: Commands,
) {
    for (entity, mut decay_over_time, mut shape_fill) in query.iter_mut() {
        decay_over_time.timer.tick(time.delta());

        if let Some(shape_fill) = shape_fill.as_mut() {
            shape_fill
                .color
                .set_alpha(1.0 - decay_over_time.timer.fraction())
        }

        if decay_over_time.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn on_place_coins_request_system(
    trigger: Trigger<RequestToPlaceCoins>,
    mut commands: Commands,
    game_resources: Res<GameResources>,
    mut rng: GlobalEntropy<WyRand>,
    spatial_query: SpatialQuery,
) {
    let mut spawned_coins = 0;
    let mut mask = LayerMask::ALL;
    mask.remove(GameLayer::GadgetFieldsLayer);
    let filter = SpatialQueryFilter::from_mask(mask);
    for _ in 0..100 {
        let position = game_resources.get_random_position_in_play_area(&mut rng);
        let intersections = spatial_query.shape_intersections(
            &CollectibleType::CoinType.collider(),
            position,
            0.0,
            &filter,
        );

        if !intersections.is_empty() {
            continue;
        }

        commands.spawn((
            CollectibleType::coin_bundle(),
            Transform::from_translation(position.extend(0.0)),
        ));
        spawned_coins += 1;

        if spawned_coins >= trigger.amount {
            break;
        }
    }
}
