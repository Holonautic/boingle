use crate::PlayerBall;
use crate::gadgets::components::{
    CollectibleType, Gadget, GadgetDeactivated, GadgetType, PlayerPlacedGadget, Preview,
};
use crate::gadgets::large_block_bundle;
use crate::gadgets::resources::GameResources;
use crate::gameplay::components::*;
use crate::gameplay::events::{OnCoinCollected, OnGadgetCardSelected};
use crate::gameplay::game_states::LevelState;
use crate::general::resources::GameCursor;
use avian2d::parry::shape::Ball;
use avian2d::parry::simba::scalar::SupersetOf;
use avian2d::prelude::*;
use bevy::asset::AssetServer;
use bevy::color::palettes::tailwind;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_simple_subsecond_system::hot;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;

pub fn basic_setup(
    mut commands: Commands,
    game_cursor: Res<GameCursor>,
    asset_resources: Res<GameResources>,
) {
    commands.spawn((Name::new("Player"), Player::new(5)));
}

#[hot]
pub fn widget_placement_system(
    mut commands: Commands,
    game_cursor: Res<GameCursor>,
    mut mouse_scroll_event: EventReader<MouseWheel>,
    input: Res<ButtonInput<MouseButton>>,
    mut player: Single<&mut Player>,
    mut q_gadget: Query<(Entity, &mut Transform, &mut Sprite, &Collider)>,
    spatial_query: SpatialQuery,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    let Some(current_widget) = player.current_widget else {
        return;
    };

    let Ok((widget_entity, mut widget_transform, mut sprite, collider)) =
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
    mut q_spitter: Query<(&mut BallSpitter, &Children), Without<IndicatorGauge>>,
    mut q_indicator: Query<(&mut Transform), With<IndicatorGauge>>,
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
) {
    for entity in previous_setup.iter() {
        commands.entity(entity).despawn();
    }
    let z_position = 50.0;

    let card_1_id = spawn_widget_card(GadgetType::Bumper, &mut commands, &shapes, &gadget_resource);
    commands
        .entity(card_1_id)
        .insert((DestroyOnHot, Transform::from_xyz(-300.0, 0.0, z_position)));

    let card_2_id = spawn_widget_card(
        GadgetType::LargeBlock,
        &mut commands,
        &shapes,
        &gadget_resource,
    );
    commands
        .entity(card_2_id)
        .insert((DestroyOnHot, Transform::from_xyz(0.0, 0.0, z_position)));

    let card_3_id = spawn_widget_card(
        GadgetType::CoinBumper,
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
            next_state.set(LevelState::PlaceWidget)
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
    mut gadget_deactivated_query: Query<(&mut Sprite), With<Gadget>>,
) {
    let Ok(mut sprite) = gadget_deactivated_query.get_mut(trigger.target()) else {
        return;
    };
    sprite.color = Color::WHITE;
}
pub fn on_gadget_deactivated_added(
    trigger: Trigger<OnAdd, GadgetDeactivated>,
    mut gadget_deactivated_query: Query<(&mut Sprite), With<Gadget>>,
) {
    let Ok(mut sprite) = gadget_deactivated_query.get_mut(trigger.target()) else {
        return;
    };
    sprite.color = tailwind::GRAY_700.into();
}

pub fn reactivate_gadgets(
    mut commands: Commands,
    mut gadgets_query: Query<(Entity, &mut Gadget)>) {
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
) {
    for entity in q_gadgets.iter() {
        commands.entity(entity).despawn();
    }
    for entity in collectible_query.iter() {
        commands.entity(entity).despawn();
    }

    player.reset();

    next_state.set(LevelState::PlaceWidget);
}
