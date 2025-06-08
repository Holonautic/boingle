use crate::gadgets::components::GadgetType;
use crate::gadgets::resources::GameResources;
use crate::gameplay::components::CardBorder;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::ecs::component::HookContext;
use bevy::ecs::system::*;
use bevy::ecs::world::DeferredWorld;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::TextBounds;
use bevy_bundled_observers::observers;
use bevy_simple_subsecond_system::prelude::*;
use bevy_vector_shapes::prelude::*;
use crate::game_ui::components::Forbidden;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy, Reflect, Default)]
pub enum ShopCardType {
    #[default]
    OneMoreBallCard,
    MoreBallsCard,
    SquareBlockCard,
    WideBlockCard,
    BumperCard,
    CoinBumperCard,
    HighFrictionBlockCard,
    MagnetCard,
    ReactivateLaserBridgeCard,
    GravityReverserCard,
    MultiBallCard,
    RecycleGadgetCard,
    RearrangeGadgetCard,
}

impl ShopCardType {
    pub fn get_gadget_type(&self) -> Option<GadgetType> {
        match self {
            ShopCardType::SquareBlockCard => Some(GadgetType::SquareBlock),
            ShopCardType::WideBlockCard => Some(GadgetType::WideBlock),
            ShopCardType::BumperCard => Some(GadgetType::Bumper),
            ShopCardType::CoinBumperCard => Some(GadgetType::CoinBumper),
            _ => None,
        }
    }
}

#[derive(Component, Debug, Clone, Hash, PartialEq, Eq, Copy, Reflect, HotPatchMigrate, Default)]
#[require(Transform, Visibility::Visible)]
#[require(Name::new("ShopCard"))]
#[require(Collider::rectangle(130.0, 190.0))]
#[reflect(Component, Default, HotPatchMigrate)]
#[component(on_add=ShopCard::on_add)]
pub struct ShopCard {
    pub card_type: ShopCardType,
}

impl ShopCard {
    pub fn new(card_type: ShopCardType) -> Self {
        Self {card_type }
    }

    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let card = world.get::<ShopCard>(context.entity).unwrap().card_type.clone();
        let game_resource = world.get_resource::<GameResources>().unwrap();
        let title = game_resource.card_title(&card);
        let description = game_resource.card_description(&card);

        world
            .commands()
            .entity(context.entity)
            .insert(observers![Self::on_hover, Self::on_out]);

        world.commands().spawn((
            ChildOf(context.entity),
            Transform::from_xyz(0.0, 40.0, 10.0),
            CardIcon(card),
        ));

        world.commands().spawn((
            ChildOf(context.entity),
            Transform::from_xyz(-55.0, -20.0, 10.0),
            Text2d::new(title),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextBounds::from(Vec2::new(110.0, 40.0)),
            Anchor::TopLeft,
            TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
        ));

        world.commands().spawn((
            ChildOf(context.entity),
            Text2d::new(description),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            TextBounds::from(Vec2::new(110.0, 40.0)),
            Anchor::TopLeft,
            TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
            Transform::from_xyz(-55.0, -60.0, 10.0),
        ));

        world.commands().queue(move |world: &mut World| {
            world
                .run_system_once(move |mut shapes: ShapeCommands| {
                    shapes.cap = Cap::Round;
                    shapes.thickness = 10.0;
                    shapes.corner_radii = Vec4::splat(15.0);
                    shapes.hollow = false;
                    shapes.color = tailwind::GRAY_800.into();
                    shapes.translate(Vec3::Z * -10.0);
                    shapes
                        .rect(Vec2::new(140., 200.))
                        .insert(ChildOf(context.entity));

                    shapes.color = tailwind::GRAY_100.into();
                    shapes.hollow = true;
                    shapes
                        .rect(Vec2::new(140., 200.))
                        .insert((ChildOf(context.entity), CardBorder));
                })
                .unwrap();
        });
    }

    pub fn on_hover(
        trigger: Trigger<Pointer<Over>>,
        q_children: Query<&Children>,
        mut q_card_border: Query<&mut ShapeFill, With<CardBorder>>,
        forbidden_querry: Query<&Forbidden>,
    ) {
        let color = if  forbidden_querry.contains(trigger.target) {Color::from(tailwind::RED_700)} else {
            Color::from(tailwind::GRAY_600)
        };
        for child in q_children.get(trigger.target).unwrap().iter() {
            if let Ok(mut shape_fill) = q_card_border.get_mut(child) {
                shape_fill.color = color;
                return;
            }
        }
    }

    pub fn on_out(
        trigger: Trigger<Pointer<Out>>,
        q_children: Query<&Children>,
        mut q_card_border: Query<&mut ShapeFill, With<CardBorder>>,
    ) {
        for child in q_children.get(trigger.target).unwrap().iter() {
            if let Ok(mut shape_fill) = q_card_border.get_mut(child) {
                shape_fill.color = tailwind::GRAY_100.into();
                return;
            }
        }
    }
}

#[derive(Component, Debug, Clone)]
#[component(on_add=CardIcon::on_add)]
pub struct CardIcon(ShopCardType);

impl CardIcon {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let game_resources = world.get_resource::<GameResources>().unwrap();
        let icon = world.get::<CardIcon>(context.entity).unwrap().clone();
        let default_size = 50.0;

        match icon.0 {
            ShopCardType::OneMoreBallCard => {}
            ShopCardType::MoreBallsCard => {}
            ShopCardType::SquareBlockCard { .. } => {
                let slice_border = 30.0;
                let image_mode = SpriteImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(slice_border),
                    center_scale_mode: SliceScaleMode::Stretch,
                    ..default()
                });
                let image = game_resources.gadget_images[&GadgetType::SquareBlock].clone();
                world.commands().entity(context.entity).insert((Sprite {
                    image,
                    image_mode,
                    custom_size: Some(Vec2::new(default_size, default_size)),
                    ..default()
                },));
            }

            ShopCardType::WideBlockCard { .. } => {
                let slice_border = 30.0;
                let image_mode = SpriteImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(slice_border),
                    center_scale_mode: SliceScaleMode::Stretch,
                    ..default()
                });
                let image = game_resources.gadget_images[&GadgetType::WideBlock].clone();
                world.commands().entity(context.entity).insert((Sprite {
                    image,
                    image_mode,
                    custom_size: Some(Vec2::new(80.0, 40.0)),
                    ..default()
                },));
            }
            ShopCardType::BumperCard { .. } => {
                let image = game_resources.gadget_images[&GadgetType::Bumper].clone();
                world.commands().entity(context.entity).insert((Sprite {
                    image,
                    custom_size: Some(Vec2::new(default_size, default_size)),
                    ..default()
                },));
            }
            ShopCardType::CoinBumperCard { .. } => {
                let image = game_resources.gadget_images[&GadgetType::CoinBumper].clone();
                world.commands().entity(context.entity).insert((Sprite {
                    image,
                    custom_size: Some(Vec2::new(default_size, default_size)),
                    ..default()
                },));
            }
            ShopCardType::HighFrictionBlockCard => {}
            ShopCardType::MagnetCard { .. } => {}
            ShopCardType::ReactivateLaserBridgeCard => {}
            ShopCardType::GravityReverserCard => {}
            ShopCardType::MultiBallCard => {}
            ShopCardType::RecycleGadgetCard => {}
            ShopCardType::RearrangeGadgetCard => {}
        }
    }
}

#[derive(Component, Debug, Clone, Event)]
#[component(on_add=OnClickOnShopCard::on_add)]
pub struct OnClickOnShopCard;

impl OnClickOnShopCard {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        world.commands().entity(context.entity).insert(
            observers![Self::on_click]
        );
    }

    fn on_click(trigger: Trigger<Pointer<Click>>, mut commands: Commands) {
        commands.trigger_targets(OnClickOnShopCard, trigger.target);
    }
}
