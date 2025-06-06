use crate::PlayerBall;
use crate::gadgets::resources::GameResources;
use crate::gadgets::*;
use crate::gameplay::events::OnCoinCollected;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_bundled_observers::observers;
use std::fmt::{Display, Formatter};

#[derive(Component, Debug, Clone, Reflect, Hash, PartialEq, Eq)]
pub enum GadgetType {
    LargeBlock,
    Bumper,
    CoinBumper,
    // Cannon,
}

impl GadgetType {
    pub fn card_icon_size(&self) -> Vec3 {
        match self {
            GadgetType::LargeBlock => Vec3::splat(0.3),
            GadgetType::Bumper => Vec3::splat(1.0),
            GadgetType::CoinBumper => Vec3::splat(0.5),
        }
    }

    pub fn spawn_widget(
        &self,
        commands: &mut Commands,
        asset_resource: &GameResources,
    ) -> Entity {
        match self {
            GadgetType::LargeBlock => commands.spawn(large_block_bundle(asset_resource)).id(),
            GadgetType::Bumper => commands.spawn(Bumper::bundle(asset_resource)).id(),
            GadgetType::CoinBumper => commands.spawn(coin_bumper_bundle(asset_resource)).id(),
        }
    }
}

impl Display for GadgetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GadgetType::LargeBlock => write!(f, "Large Block"),
            GadgetType::Bumper => write!(f, "Bumper"),
            GadgetType::CoinBumper => write!(f, "Coin Bumper"),
        }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Preview;

#[derive(Component, Debug, Reflect)]
pub struct PlayerPlacedGadget;

#[derive(Component, Debug, Reflect)]
pub struct Gadget {
    pub activations_left: usize,
    pub activations_per_round: usize,
}

impl Gadget {
    pub fn new(activations: usize) -> Self {
        Self {
            activations_per_round: activations,
            activations_left: activations,
        }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Bumper{
    pub points: usize,
}

impl Bumper {
    pub fn new(points: usize) -> Self {
        Self { points }
    }

    pub fn bundle(gadget_image_resource: &GameResources) -> impl Bundle {
        let image_handle = gadget_image_resource
            .gadget_images
            .get(&GadgetType::Bumper)
            .unwrap()
            .clone();
        (
            Bumper::new(5),
            Gadget::new(1),
            Name::new("bumper"),
            Transform::from_scale(Vec3::splat(1.0)),
            Sprite::from_image(image_handle),
            RigidBody::Static,
            Restitution::new(0.9),
            Collider::circle(35.0),
            CollisionEventsEnabled,
            observers![on_bumper_hit],
        )
    }
}



#[derive(Component, Debug, Reflect)]
pub struct CoinBumperGadget {
    pub coins_to_spawn: usize,
}

impl CoinBumperGadget {
    pub fn new(coins_to_spawn: usize) -> Self {
        Self { coins_to_spawn }
    }
}

#[derive(Component, Debug, Clone, Reflect, Hash, PartialEq, Eq)]
pub enum CollectibleType {
    Coin,
}

impl CollectibleType {
    pub fn collider(&self) -> Collider {
        match self {
            CollectibleType::Coin => Collider::circle(10.0),
        }
    }

    pub fn coin_bundle(asset_resource: &GameResources, position: Vec3) -> impl Bundle {
        let image = asset_resource
            .collectibles_images
            .get(&CollectibleType::Coin)
            .unwrap()
            .clone();
        (
            CollectibleType::Coin,
            Name::new("coin"),
            CollectibleType::Coin.collider(),
            Transform::from_translation(position),
            CollisionEventsEnabled,
            observers![|trigger: Trigger<OnCollisionStart>,
                        mut commands: Commands,
                        ball_query: Query<Entity, With<PlayerBall>>| {
                if ball_query.get(trigger.collider).is_err() {
                    return;
                }
                commands.trigger_targets(OnCoinCollected, trigger.target());
            }],
            children![(
                Transform::from_scale(Vec3::splat(0.30)),
                Sprite::from_image(image),
            )],
        )
    }
}
