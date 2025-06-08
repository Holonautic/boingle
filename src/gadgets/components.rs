use crate::gadgets::resources::GameResources;
use crate::gadgets::*;
use crate::gameplay::components::{BallCannon, DestroyOnStandingStill};
use crate::gameplay::events::*;
use crate::general::components::SpriteVisualOf;
use avian2d::parry::na::DimAdd;
use avian2d::prelude::*;
use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::TextBounds;
use bevy_bundled_observers::observers;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Component, Debug, Clone, Reflect, Hash, PartialEq, Eq, Copy)]
pub enum GadgetType {
    SquareBlock,
    WideBlock,
    Bumper,
    CoinBumper,
    BallCannon,
}

impl GadgetType {
    pub fn spawn_widget(&self, commands: &mut Commands, asset_resource: &GameResources) -> Entity {
        match self {
            GadgetType::SquareBlock => commands
                .spawn((SquareBlock, Gadget::new(5), PointsOnHit::new(1)))
                .id(),
            GadgetType::WideBlock => commands
                .spawn((WideBlock, Gadget::new(5), PointsOnHit::new(1)))
                .id(),
            GadgetType::Bumper => commands
                .spawn((Bumper, Gadget::new(3), PointsOnHit::new(3)))
                .id(),
            GadgetType::CoinBumper => commands.spawn(CoinBumperGadget::default()).id(),
            GadgetType::BallCannon => commands.spawn(BallCannon::bundle()).id(),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            GadgetType::SquareBlock => "Points: 5x1",
            GadgetType::WideBlock => "Points: 5x1",
            GadgetType::Bumper => "Points: 3x3",
            GadgetType::CoinBumper => "Spawns Coins",
            GadgetType::BallCannon => "Click to Spawns Ball",
        }
    }
}

impl Display for GadgetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GadgetType::SquareBlock => f.write_str("Square Block"),
            GadgetType::WideBlock => write!(f, "Wide Block"),
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
#[require(Transform)]
#[require(Visibility)]
#[require(Gadget::new(1))]
#[require(PointsOnHit::new(5))]
#[require(Name::new("bumper"))]
#[require(RigidBody::Static)]
#[require(BounceOnHit)]
#[require(Restitution::new(1.5))]
#[require(Collider::circle(30.0))]
#[require(CollisionEventsEnabled)]
#[component(on_add=Bumper::on_add)]
pub struct Bumper;

impl Bumper {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let game_resources = world.resource::<GameResources>();
        let image = game_resources.gadget_images[&GadgetType::Bumper].clone();
        world.commands().spawn((
            ChildOf(context.entity),
            SpriteVisualOf(context.entity),
            Transform::from_scale(Vec3::splat(0.5)),
            Sprite::from_image(image),
        ));
    }
}

#[derive(Component, Debug, Reflect)]
pub struct GadgetDeactivated;

#[derive(Component, Debug, Reflect)]
#[require(Transform, Visibility)]
#[require(Name::new("coin_bumper"))]
#[require(GadgetType::CoinBumper)]
#[require(Gadget::new(1))]
#[require(RigidBody::Static)]
#[require(Restitution::new(2.0))]
#[require(Collider::circle(29.0))]
#[require(CollisionEventsEnabled)]
#[require(BounceOnHit)]
#[component(on_add=CoinBumperGadget::on_add)]
pub struct CoinBumperGadget {
    pub coins_to_spawn: usize,
}
impl Default for CoinBumperGadget {
    fn default() -> Self {
        Self { coins_to_spawn: 3 }
    }
}
impl CoinBumperGadget {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let game_resources = world.resource::<GameResources>();
        let image = game_resources.gadget_images[&GadgetType::CoinBumper].clone();

        world
            .commands()
            .entity(context.entity)
            .insert(observers![on_coins_spawn_from_bumper]);

        world.commands().spawn((
            ChildOf(context.entity),
            SpriteVisualOf(context.entity),
            Transform::from_scale(Vec3::splat(0.5)),
            Sprite::from_image(image),
        ));
    }
}

#[derive(Component, Debug, Clone, Reflect, Hash, PartialEq, Eq)]
#[require(RemainingRounds(3))]
#[require(ShrinkAtEndOfRound(0.3))]
pub enum CollectibleType {
    CoinType,
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
        let image = game_resource.collectibles_images[&CollectibleType::CoinType].clone();
        let mut sprite = world.get_mut::<Sprite>(context.entity).unwrap();

        sprite.image = image;

        world
            .commands()
            .spawn((ChildOf(context.entity), SpriteVisualOf(context.entity)));
    }
}

impl CollectibleType {
    pub fn collider(&self) -> Collider {
        match self {
            CollectibleType::CoinType => Collider::circle(14.0),
        }
    }

    pub fn coin_bundle() -> impl Bundle {
        (
            CollectibleType::CoinType,
            Coin::new(1),
            Name::new("coin"),
            CollectibleType::CoinType.collider(),
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
#[require(Collider::circle(25.0))]
#[require(Name::new("player_ball"))]
#[require(DestroyOnStandingStill::new(1.0, Duration::from_secs_f32(5.0)))]
#[component(on_add=PlayerBall::on_add)]
pub struct PlayerBall;

impl PlayerBall {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let game_resource = world.get_resource::<GameResources>().unwrap();
        let image = game_resource.ball_image.clone();

        world
            .commands()
            .entity(context.entity)
            .insert(Sprite::from_image(image));
    }
}

#[derive(Component, Debug, Reflect)]
#[require(Name::new("block"))]
#[require(RigidBody::Static)]
#[require(Restitution::new(0.7))]
#[require(Collider)]
#[require(Visibility)]
#[require(Transform)]
#[require(PointsOnHit::new(3))]
#[require(Gadget::new(3))]
#[require(CollisionEventsEnabled)]
#[component(on_add=Block::on_add)]
pub struct Block {
    pub size: Vec2,
}

impl Block {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }

    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let block = world.get::<Block>(context.entity).unwrap();
        let size = block.size;
        let game_resource = world.get_resource::<GameResources>().unwrap();
        let image = game_resource.gadget_images[&GadgetType::SquareBlock].clone();
        let slice_border = 30.0;
        let scale_mode = SpriteImageMode::Sliced(TextureSlicer {
            border: BorderRect::all(slice_border),
            center_scale_mode: SliceScaleMode::Stretch,
            ..default()
        });
        world.commands().spawn((
            ChildOf(context.entity),
            SpriteVisualOf(context.entity),
            Sprite {
                image,
                custom_size: Some(size),
                image_mode: scale_mode,
                ..default()
            },
        ));
        world
            .commands()
            .entity(context.entity)
            .insert((Collider::rectangle(size.x, size.y),));
    }
}

#[derive(Component, Debug, Reflect)]
#[component(on_add=PointsOnHit::on_add)]
pub struct PointsOnHit {
    pub amount: usize,
}
impl PointsOnHit {
    pub fn new(amount: usize) -> Self {
        Self { amount }
    }

    pub fn on_add(mut world: DeferredWorld, context: HookContext) {
        world
            .commands()
            .entity(context.entity)
            .insert(observers![on_hit_gain_points]);
    }
}

#[derive(Component, Debug, Reflect, Default)]
#[component(on_add=BounceOnHit::on_add)]
pub struct BounceOnHit;

impl BounceOnHit {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        world
            .commands()
            .entity(context.entity)
            .insert(observers![on_hit_bounce]);
    }
}

#[derive(Component, Debug, Reflect)]
#[require(Name::new("wide_block"))]
#[require(Block::new(Vec2::new(150.0, 40.0)))]
#[require(Gadget::new(3))]
pub struct WideBlock;

#[derive(Component, Debug, Reflect)]
#[require(Name::new("square_block"))]
#[require(Block::new(Vec2::new(50.0, 50.0)))]
#[require(Gadget::new(5))]
pub struct SquareBlock;

#[derive(Component, Debug, Reflect, Deref, DerefMut)]
pub struct ShrinkAtEndOfRound(pub f32);

#[derive(Component, Debug, Reflect, Deref, DerefMut)]
pub struct RemainingRounds(pub usize);

pub struct ActivationTime {
    pub time: Duration,
}
impl ActivationTime {
    pub fn new(seconds: f32) -> Self {
        Self {
            time: Duration::from_secs_f32(seconds),
        }
    }
}
