use crate::gadgets::components::*;
use crate::gadgets::resources::*;
use crate::gameplay::components::*;
use crate::general::components::SpriteVisual;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_easings::{Ease, EasingComponent};
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::WyRand;
use bevy_simple_subsecond_system::hot;
use rand::Rng;
use crate::game_ui::components::FloatingScore;

#[hot]
pub fn on_coins_spawn_from_bumper(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    mut q_gadget: Query<(&mut Gadget, &CoinBumperGadget), Without<GadgetDeactivated>>,
    ball_query: Query<Entity, With<PlayerBall>>,
    mut rng: GlobalEntropy<WyRand>,
    spatial_query: SpatialQuery,
    asset_resources: Res<GameResources>,
) {
    if ball_query.get(trigger.collider).is_err() {
        return;
    }
    let Ok((mut gadget, coin_bumper)) = q_gadget.get_mut(trigger.target()) else {
        return;
    };

    gadget.activations_left -= 1;

    if gadget.activations_left == 0 {
        commands
            .entity(trigger.target())
            .try_insert(GadgetDeactivated);
    }

    let mut spawned_coins = 0;

    while spawned_coins < coin_bumper.coins_to_spawn {
        let x_min_max = asset_resources.play_area.x as i32;
        let y_min_max = asset_resources.play_area.y as i32;
        let x = rng.random_range(-x_min_max..=x_min_max) as f32;
        let y = rng.random_range(-y_min_max..=y_min_max) as f32;
        let position = Vec2::new(x, y);
        let intersections = spatial_query.shape_intersections(
            &CollectibleType::Coin.collider(),
            position,
            0.0,
            &SpatialQueryFilter::default(),
        );

        if !intersections.is_empty() {
            continue;
        }

        commands.spawn((
            CollectibleType::coin_bundle(),
            Transform::from_translation(position.extend(0.0)),
        ));
        spawned_coins += 1;
    }
}

pub fn on_hit_gain_points(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    mut hit_query: Query<(Entity, &Transform, &PointsOnHit, &mut Gadget)>,
    q_ball: Query<Entity, With<PlayerBall>>,
    mut player: Single<&mut Player>,
) {
    if q_ball.get(trigger.collider).is_err() {
        return;
    }
    let Ok((entity, transform, points_on_hit, mut gadget)) = hit_query.get_mut(trigger.target())
    else {
        return;
    };

    if gadget.activations_left > 0 {
        gadget.activations_left -= 1;
        player.points += points_on_hit.amount;

        commands.spawn((
            Transform::from_translation(transform.translation),
            FloatingScore(points_on_hit.amount),
        ));

        if gadget.activations_left == 0 {
            commands.entity(entity).insert(GadgetDeactivated);
        }
    }
}

pub fn on_hit_bounce(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    mut bounce_on_hit_query: Query<&SpriteVisual, (With<BounceOnHit>, Without<EasingComponent<Transform>>)>,
    mut transform_query: Query<&mut Transform>,
    q_ball: Query<Entity, With<PlayerBall>>,
) {
    if q_ball.get(trigger.collider).is_err() {
        return;
    }
    let Ok(sprite_visual) = bounce_on_hit_query.get_mut(trigger.target()) else {
        return;
    };

    let sprite_transform = transform_query.get_mut(**sprite_visual).unwrap();
    let start_scale = sprite_transform.scale;
    let duration = 0.01;
    commands.entity(**sprite_visual).insert(
        sprite_transform
            .ease_to_fn(
                |start| Transform {
                    scale: start.scale * 1.1,
                    ..*start
                },
                bevy_easings::EaseFunction::QuadraticOut,
                bevy_easings::EasingType::Once {
                    duration: std::time::Duration::from_secs_f32(duration),
                },
            )
            .ease_to_fn(
                |start| Transform {
                    scale: start_scale,
                    ..*start
                },
                bevy_easings::EaseFunction::QuadraticOut,
                bevy_easings::EasingType::Once {
                    duration: std::time::Duration::from_secs_f32(duration),
                },
            ),
    );
}

pub fn on_finish_easing_destroy(
    trigger: Trigger<OnRemove, EasingComponent<Transform>>,
    mut commands: Commands,
) {
    commands.entity(trigger.target()).despawn();
}
