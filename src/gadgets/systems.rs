use crate::gadgets::components::*;
use crate::gadgets::resources::*;
use crate::game_ui::components::FloatingScore;
use crate::gameplay::components::*;
use crate::general::components::SpriteVisual;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_bundled_observers::observers;
use bevy_easings::{Ease, EaseFunction, EasingComponent, EasingType};
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::WyRand;
use bevy_simple_subsecond_system::hot;
use bevy_vector_shapes::prelude::*;
use rand::Rng;
use std::time::Duration;
use bevy::ecs::system::command::trigger;
use crate::gameplay::events::RequestToPlaceCoins;

#[hot]
pub fn on_coins_spawn_from_bumper(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    mut q_gadget: Query<(&mut Gadget, &CoinBumperGadget), Without<GadgetDeactivated>>,
    ball_query: Query<Entity, With<PlayerBall>>,
) {
    if ball_query.get(trigger.collider).is_err() {
        return;
    }
    let Ok((mut gadget, coin_bumper)) = q_gadget.get_mut(trigger.target()) else {
        return;
    };

    commands.trigger(RequestToPlaceCoins::new(coin_bumper.coins_to_spawn));
    gadget.activations_left -= 1;

    if gadget.activations_left == 0 {
        commands
            .entity(trigger.target())
            .try_insert(GadgetDeactivated);
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
        player.points_this_round += points_on_hit.amount;
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
    mut bounce_on_hit_query: Query<
        (&SpriteVisual, &mut BounceOnHit),
        Without<EasingComponent<Transform>>,
    >,
    mut transform_query: Query<&mut Transform>,
    q_ball: Query<Entity, With<PlayerBall>>,
) {
    if q_ball.get(trigger.collider).is_err() {
        return;
    }
    let Ok((sprite_visual, mut bounce_on_hit)) = bounce_on_hit_query.get_mut(trigger.target())
    else {
        return;
    };

    let sprite_transform = transform_query.get_mut(**sprite_visual).unwrap();
    if bounce_on_hit.starting_size.is_none() {
        bounce_on_hit.starting_size = Some(sprite_transform.scale);
    }
    let start_scale = bounce_on_hit.starting_size.unwrap();
    let duration = 0.01;
    commands.entity(**sprite_visual).insert(
        sprite_transform
            .ease_to_fn(
                |start| Transform {
                    scale: start_scale * 1.1,
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
    commands.entity(trigger.target()).try_despawn();
}

pub fn gravity_inverse_field_system(
    time: Res<Time>,
    mut inverse_gravity_field_query: Query<(&mut GravityReverseField, &Transform)>,
    mut shapes: ShapeCommands,
    mut rng: GlobalEntropy<WyRand>,
) {
    for (mut gravity_field, transform) in inverse_gravity_field_query.iter_mut() {
        gravity_field.time_since_last_emission += time.delta();
        let half_width = 50.0;
        let half_height = 200.0;
        if gravity_field.time_since_last_emission.as_secs_f32() > 0.1 {
            for _ in 0..rng.random_range(1..=2) {
                let x_offset = rng.random_range(-half_width..=half_width);
                gravity_field.time_since_last_emission = Duration::default();
                let mut transform = transform.clone();
                transform.translation.x += x_offset;
                transform.translation.y -= half_height;

                let easing = transform.ease_to_fn(
                    |start| Transform {
                        translation: transform.translation + transform.up() * 2.0 * half_height,
                        ..*start
                    },
                    EaseFunction::QuadraticOut,
                    EasingType::Once {
                        duration: Duration::from_millis(1000),
                    },
                );
                shapes.color = Color::from(tailwind::BLUE_500);
                shapes.rect(Vec2::new(1.0, 10.0)).insert((
                    transform,
                    easing,
                    observers![on_finish_easing_destroy],
                ));
            }
        }
    }
}

pub fn on_entering_reverse_gravity_field(
    trigger: Trigger<OnCollisionStart>,
    mut ball_gravity_query: Query<&mut GravityScale, With<PlayerBall>>,
) {
    let Ok(mut gravity) = ball_gravity_query.get_mut(trigger.collider) else {
        return;
    };

    gravity.0 = -3.0;
}

pub fn on_exiting_reverse_gravity_field(
    trigger: Trigger<OnCollisionEnd>,
    mut ball_gravity_query: Query<&mut GravityScale, With<PlayerBall>>,
) {
    let Ok(mut gravity) = ball_gravity_query.get_mut(trigger.collider) else {
        return;
    };
    gravity.0 = 1.0;
}
