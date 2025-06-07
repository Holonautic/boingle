use crate::gadgets::components::*;
use crate::gadgets::resources::GameResources;
use crate::gameplay::events::OnGadgetCardSelected;
use crate::gameplay::game_states::LevelState;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::TextBounds;
use bevy_bundled_observers::observers;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;
use bevy_rand::prelude::*;
use rand::prelude::SliceRandom;

#[derive(Component, Debug, Reflect, Default)]
pub struct Player {
    pub current_widget: Option<Entity>,
    pub current_hand: Vec<GadgetType>,
    pub widget_deck: Vec<GadgetType>,
    pub discard_pile: Vec<GadgetType>,
    starter_deck:  Vec<GadgetType>,
    pub points: usize,
    pub points_last_round: usize,
    pub coins: usize,
    pub balls_left: usize,
    pub balls_per_level: usize,
    pub current_level: usize,
    pub point_for_next_level: usize,
}

impl Player {
    pub fn new(balls_per_level: usize, rng: &mut Entropy<WyRand>) -> Self {
        let starter_deck = vec![
            GadgetType::WideBlock,
            GadgetType::WideBlock,
            GadgetType::WideBlock,
            GadgetType::WideBlock,
            GadgetType::SquareBlock,
            GadgetType::SquareBlock,
            GadgetType::SquareBlock,
            GadgetType::SquareBlock,
            GadgetType::Bumper,
            GadgetType::Bumper,
            GadgetType::CoinBumper,
        ];
        let mut widget_deck = starter_deck.clone();
        widget_deck.shuffle(rng);
        
        Self {
            balls_left: balls_per_level,
            balls_per_level,
            starter_deck,
            widget_deck,
            point_for_next_level: Player::points_for_level(0),
            ..default()
        }

    }

    pub fn points_for_level(level: usize) -> usize {
        let base = 15.0_f32;
        let growth_factor = 2.2_f32;
        (base * growth_factor.powi(level.saturating_sub(1) as i32)).round() as usize
    }
    pub fn next_card(&mut self, rng: &mut Entropy<WyRand>) -> GadgetType {
        if self.widget_deck.is_empty() {
            self.reshuffle_deck(rng)
        }
        self.widget_deck.pop().unwrap()
    }
    pub fn shuffle_deck(&mut self, rng: &mut Entropy<WyRand>) {
        self.widget_deck.shuffle(rng)
    }

    pub fn reshuffle_deck(&mut self, rng: &mut Entropy<WyRand>) {
        for card in self.discard_pile.iter() {
            self.widget_deck.push(card.clone());
        }
        self.discard_pile.clear();
        self.shuffle_deck(rng)
    }

    pub fn reset(&mut self, rng: &mut Entropy<WyRand>) {
        self.current_widget = None;
        self.points = 0;
        self.coins = 0;
        self.balls_left = self.balls_per_level;

        self.widget_deck = self.starter_deck.clone();
        self.widget_deck.shuffle(rng);
        self.discard_pile.clear();
        self.current_hand.clear();

    }
}
#[derive(Component, Debug)]
#[relationship(relationship_target = HelperText)]
pub struct HelpTextFor(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = HelpTextFor)]
pub struct HelperText(Vec<Entity>);

#[allow(unused)]
impl HelperText {
    pub fn get_helper_texts(&self) -> &[Entity] {
        &self.0
    }
}
#[derive(Component, Debug, Reflect)]
pub struct BallSpawnPoint;

#[derive(Component, Debug, Reflect)]
#[require(Transform, Visibility)]
#[component(on_insert=BallCannon::on_ball_cannon_added)]
pub struct BallCannon {
    pub power: f32,
    pub max_power: f32,
    pub gain: f32,
    pub is_increasing_power: bool,
}

impl Default for BallCannon {
    fn default() -> Self {
        BallCannon {
            power: 1000.0,
            max_power: 1500.0,
            gain: 1000.0,
            is_increasing_power: false,
        }
    }
}
impl BallCannon {
    fn on_ball_cannon_added(mut world: DeferredWorld, context: HookContext) {
        world.commands().queue(move |world: &mut World| {
            let game_resources = world.get_resource::<GameResources>().unwrap();
            let image = game_resources.gadget_images[&GadgetType::BallCannon].clone();
            world.commands().spawn((
                ChildOf(context.entity),
                Transform::from_scale(Vec3::splat(0.25))
                    .with_rotation(Quat::from_rotation_z(TAU * 0.5)),
                Sprite::from_image(image),
            ));
        });
    }

    pub fn bundle() -> impl Bundle {
        (
            BallCannon::default(),
            Collider::rectangle(25.0, 50.0),
            RigidBody::Static,
            observers![BallCannon::on_pressed, BallCannon::on_released],
        )
    }

    fn on_pressed(
        _trigger: Trigger<Pointer<Pressed>>,
        mut commands: Commands,
        // mut q_spitter: Query<&mut BallSpitter>,
        q_balls: Query<Entity, With<PlayerBall>>,
    ) {
        for ball_entity in q_balls.iter() {
            commands.entity(ball_entity).despawn();
        }
        // let mut spitter = q_spitter.get_mut(trigger.target).unwrap();
        // spitter.is_increasing_power = true;
    }

    fn on_released(
        trigger: Trigger<Pointer<Released>>,
        mut commands: Commands,
        mut q_spitter: Query<(&mut BallCannon, &Transform)>,
        state: Res<State<LevelState>>,
        mut next_state: ResMut<NextState<LevelState>>,
    ) {
        if !matches!(state.get(), LevelState::ShootBall) {
            return;
        }
        let (mut spitter, spitter_transform) = q_spitter.get_mut(trigger.target).unwrap();
        spitter.is_increasing_power = false;
        if spitter.power == 0.0 {
            return;
        }
        let forward = spitter_transform.rotation * Vec3::Y;
        let forward_2d = forward.truncate();
        commands.spawn((
            PlayerBall,
            Transform::from_translation(spitter_transform.translation).with_scale(Vec3::splat(0.5)),
            LinearVelocity(forward_2d * spitter.power),
        ));
        next_state.set(LevelState::BallBouncing);
    }
}
#[derive(Component, Debug, Reflect)]
pub struct IndicatorGauge;

#[derive(Component, Debug, Reflect)]
pub struct CardBorder;

#[derive(Component, Debug, Reflect)]
pub struct GadgetCard {
    gadget_type: GadgetType,
}

pub fn spawn_widget_card(
    gadget_type: GadgetType,
    commands: &mut Commands,
    shapes: &ShapeCommands,
    gadget_resource: &GameResources,
) -> Entity {
    commands
        .spawn((
            GadgetCard {
                gadget_type: gadget_type.clone(),
            },
            Transform::default(),
            Visibility::Visible,
            Name::new("Widget Selector"),
            Collider::rectangle(130.0, 190.0),
            children![
                (
                    Transform::from_xyz(0.0, 40.0, 10.0),
                    gadget_resource.card_sprite(&gadget_type),
                ),
                (
                    Text2d::new(format!("{}", gadget_type)),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextBounds::from(Vec2::new(110.0, 40.0)),
                    Anchor::TopLeft,
                    TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                    Transform::from_xyz(-55.0, -20.0, 10.0),
                ),
                (
                    Text2d::new(format!("{}", gadget_type.description())),
                    TextFont {
                        font_size: 10.0,
                        ..default()
                    },
                    TextBounds::from(Vec2::new(110.0, 40.0)),
                    Anchor::TopLeft,
                    TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                    Transform::from_xyz(-55.0, -60.0, 10.0),
                )
            ],
        ))
        .with_shape_children(shapes.config(), |shapes| {
            shapes.cap = Cap::Round;
            shapes.thickness = 10.0;
            shapes.corner_radii = Vec4::splat(15.0);
            shapes.hollow = false;
            shapes.color = tailwind::GRAY_800.into();
            shapes.translate(Vec3::Z * -10.0);
            shapes.rect(Vec2::new(140., 200.));

            shapes.color = tailwind::GRAY_100.into();
            shapes.hollow = true;
            shapes.rect(Vec2::new(140., 200.)).insert(CardBorder);
        })
        .observe(
            |trigger: Trigger<Pointer<Over>>,
             q_children: Query<&Children>,
             mut q_card_border: Query<&mut ShapeFill, With<CardBorder>>| {
                for child in q_children.get(trigger.target).unwrap().iter() {
                    if let Ok(mut shape_fill) = q_card_border.get_mut(child) {
                        shape_fill.color = tailwind::GRAY_600.into();
                        return;
                    }
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>,
             q_children: Query<&Children>,
             mut q_card_border: Query<&mut ShapeFill, With<CardBorder>>| {
                for child in q_children.get(trigger.target).unwrap().iter() {
                    if let Ok(mut shape_fill) = q_card_border.get_mut(child) {
                        shape_fill.color = tailwind::GRAY_100.into();
                        return;
                    }
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Click>>,
             mut commands: Commands,
             q_card: Query<&GadgetCard>| {
                let gadget_card = q_card.get(trigger.target).unwrap();
                commands.trigger_targets(
                    OnGadgetCardSelected::new(gadget_card.gadget_type.clone()),
                    trigger.target,
                );
            },
        )
        .id()
}
