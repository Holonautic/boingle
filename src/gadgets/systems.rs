use crate::PlayerBall;
use crate::gadgets::components::*;
use crate::gadgets::resources::*;
use crate::gameplay::components::Player;
use avian2d::parry::shape::Ball;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_rand::global::GlobalEntropy;
use bevy_rand::prelude::WyRand;
use bevy_simple_subsecond_system::hot;
use rand::Rng;

#[hot]
pub fn on_coins_spawn_from_bumper(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    mut q_gadget: Query<(&mut Gadget, &CoinBumperGadget)>,
    ball_query: Query<Entity, With<PlayerBall>>,
    mut rng: GlobalEntropy<WyRand>,
    spatial_query: SpatialQuery,
    asset_resources: Res<GameResources>,
) {
    if ball_query.get(trigger.collider).is_err() {
        return;
    }
    let (mut gadget, coin_bumper) = q_gadget.get_mut(trigger.target()).unwrap();
    if gadget.activations_left <= 0 {
        return;
    }

    gadget.activations_left -= 1;

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

        commands.spawn(CollectibleType::coin_bundle(
            &asset_resources,
            position.extend(0.0),
        ));
        spawned_coins += 1;
    }
}

pub fn on_bumper_hit(
    trigger: Trigger<OnCollisionStart>,
    mut bumper_query: Query<(&Bumper, &mut Gadget)>,
    mut q_ball: Query<&mut LinearVelocity, With<PlayerBall>>,
    mut player: Single<&mut Player>,
) {
    let Ok((bumper, mut gadget)) = bumper_query.get_mut(trigger.target()) else {
        return;
    };
    let Ok(mut velocity) = q_ball.get_mut(trigger.collider) else {
        return;
    };

    if gadget.activations_left > 0 {
        gadget.activations_left -= 1;
        player.points += bumper.points;
        velocity.0 *= 1.5;
    }
}
