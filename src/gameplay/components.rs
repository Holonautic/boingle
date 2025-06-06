use crate::gadgets::components::GadgetType;
use crate::gadgets::resources::GameResources;
use crate::gameplay::events::OnGadgetCardSelected;
use avian2d::prelude::{Collider, OnCollisionStart};
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::TextBounds;
use bevy_vector_shapes::prelude::*;

#[derive(Component, Debug, Reflect, Default)]
pub struct Player {
    pub current_widget: Option<Entity>,
    pub widget_deck: Vec<GadgetType>,
    pub discard_pile: Vec<GadgetType>,
    pub points: usize,
    pub coins: usize,
    pub balls_left: usize,
    pub balls_per_level: usize,
}

impl Player {
    pub fn new(balls_per_level: usize) -> Self {
        Self {
            balls_left: balls_per_level,
            balls_per_level,
            ..default()
        }
    }
    
    pub fn reset(&mut self) {
        self.current_widget = None;
        self.points = 0;
        self.coins = 0;
        self.balls_left = self.balls_per_level;
    }
}
#[derive(Component, Debug)]
#[relationship(relationship_target = HelperText)]
pub struct HelpTextFor(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = HelpTextFor)]
pub struct HelperText(Vec<Entity>);

impl HelperText {
    pub fn get_helper_texts(&self) -> &[Entity] {
        &self.0
    }
}
#[derive(Component, Debug, Reflect)]
pub struct BallSpawnPoint;

#[derive(Component, Debug, Reflect)]
pub struct BallSpitter {
    pub power: f32,
    pub max_power: f32,
    pub gain: f32,
    pub is_increasing_power: bool,
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
                    Transform::from_xyz(0.0, 40.0, 10.0).with_scale(gadget_type.card_icon_size()),
                    Sprite::from_image(
                        gadget_resource
                            .gadget_images
                            .get(&gadget_type)
                            .unwrap()
                            .clone()
                    ),
                ),
                (
                    Text2d::new(format!("{}", gadget_type)),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextBounds::from(Vec2::new(110.0, 70.0)),
                    Anchor::TopLeft,
                    TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                    Transform::from_xyz(-55.0, -30.0, 10.0),
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
                mut commands: Commands,
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
        .observe(|trigger: Trigger<Pointer<Click>>, mut commands: Commands, q_card: Query<&GadgetCard>| {
            let gadget_card = q_card.get(trigger.target).unwrap();
            commands.trigger_targets(
                OnGadgetCardSelected::new(gadget_card.gadget_type.clone()),
                trigger.target,
            );
        })
        .id()
}
