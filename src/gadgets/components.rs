use crate::gadgets::resources::GameResources;
use crate::gadgets::*;
use crate::gameplay::components::BallCannon;
use crate::gameplay::events::*;
use avian2d::prelude::*;
use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy_bundled_observers::observers;
use std::fmt::{Display, Formatter};

#[derive(Component, Debug, Clone, Reflect, Hash, PartialEq, Eq)]
pub enum GadgetType {
    LargeBlock,
    Bumper,
    CoinBumper,
    BallCannon,
}

impl GadgetType {
    pub fn card_icon_size(&self) -> Vec3 {
        match self {
            GadgetType::LargeBlock => Vec3::splat(0.3),
            GadgetType::Bumper => Vec3::splat(1.0),
            GadgetType::CoinBumper => Vec3::splat(0.5),
            GadgetType::BallCannon => Vec3::splat(0.5),
        }
    }

    pub fn spawn_widget(&self, commands: &mut Commands, asset_resource: &GameResources) -> Entity {
        match self {
            GadgetType::LargeBlock => commands.spawn(large_block_bundle(asset_resource)).id(),
            GadgetType::Bumper => commands.spawn(Bumper::bundle(asset_resource)).id(),
            GadgetType::CoinBumper => commands.spawn(coin_bumper_bundle(asset_resource)).id(),
            GadgetType::BallCannon => commands.spawn(BallCannon::bundle()).id(),
        }
    }
}

impl Display for GadgetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GadgetType::LargeBlock => write!(f, "Large Block"),
            GadgetType::Bumper => write!(f, "Bumper"),
            GadgetType::CoinBumper => write!(f, "Coin Bumper"),
            GadgetType::BallCannon => write!(f, "Ball Cannon"),
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
pub struct Bumper {
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
            Sprite {
                image: image_handle,
                color: tailwind::PURPLE_400.into(),
                ..Default::default()
            },
            RigidBody::Static,
            Restitution::new(0.9),
            Collider::circle(35.0),
            CollisionEventsEnabled,
            observers![on_bumper_hit],
        )
    }
}

#[derive(Component, Debug, Reflect)]
pub struct GadgetDeactivated;

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

#[derive(Component, Debug, Clone, Reflect, Hash, PartialEq, Eq)]
#[component(on_add=Coin::on_coin_added)]
pub struct Coin {
    pub value: usize,
}
impl Coin {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
    fn on_coin_added(mut world: DeferredWorld, context: HookContext) {
        let game_resource = world.get_resource::<GameResources>().unwrap();
        let image = game_resource
            .collectibles_images
            .get(&CollectibleType::Coin)
            .unwrap()
            .clone();
        let mut sprite = world.get_mut::<Sprite>(context.entity).unwrap();

        sprite.image = image;
    }
}

impl CollectibleType {
    pub fn collider(&self) -> Collider {
        match self {
            CollectibleType::Coin => Collider::circle(10.0),
        }
    }

    pub fn coin_bundle() -> impl Bundle {
        (
            CollectibleType::Coin,
            Coin::new(1),
            Name::new("coin"),
            CollectibleType::Coin.collider(),
            CollisionEventsEnabled,
            Sprite {
                custom_size: Some(Vec2::splat(25.0)),
                ..default()
            },
            observers![|trigger: Trigger<OnCollisionStart>,
                        mut commands: Commands,
                        ball_query: Query<Entity, With<PlayerBall>>| {
                if ball_query.get(trigger.collider).is_err() {
                    return;
                }
                commands.trigger_targets(OnCoinCollected, trigger.target());
            }],
        )
    }
}

#[derive(Component, Debug, Reflect)]
#[require(RigidBody::Dynamic)]
#[require(Restitution::new(0.99))]
#[require(Collider::circle(30.0))]
#[require(Name::new("player_ball"))]
#[component(on_add=PlayerBall::on_add)]
pub struct PlayerBall;

impl PlayerBall {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let game_resource = world.get_resource::<GameResources>().unwrap();
        let image = game_resource.ball_image.clone();
        world.commands().entity(context.entity).insert(Sprite::from_image(image));

    }

    // pub fn ball_bundle(asset_server: &AssetServer) -> impl Bundle {
    //     let ball_image = asset_server.load("sprites/ball_blue_small.png");
    //     (
    //         Name::new("ball_blue_small"),
    //         PlayerBall,
    //         Sprite::from_image(ball_image),
    //         RigidBody::Dynamic,
    //         Restitution::new(0.99),
    //         Collider::circle(30.0),
    //     )
    // }
}
