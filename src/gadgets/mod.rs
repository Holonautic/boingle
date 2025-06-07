pub mod components;
pub mod resources;
mod systems;

use crate::gadgets::components::*;
use crate::gadgets::resources::GameResources;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_bundled_observers::observers;
use crate::gadgets::systems::*;

pub fn large_block_bundle(gadget_image_resource: &GameResources) -> impl Bundle {
    let image_handle = gadget_image_resource
        .gadget_images
        .get(&GadgetType::LargeBlock)
        .unwrap()
        .clone();
    (
        Name::new("block_large"),
        Transform::from_scale(Vec3::splat(0.5)),
        Sprite::from_image(image_handle),
        RigidBody::Static,
        Restitution::new(0.7),
        Collider::rectangle(125.0, 260.0),
    )
}



pub fn coin_bumper_bundle(gadget_image_resource: &GameResources) -> impl Bundle {
    let image_handle = gadget_image_resource
        .gadget_images
        .get(&GadgetType::CoinBumper)
        .unwrap()
        .clone();
    (
        GadgetType::CoinBumper,
        Gadget::new(1),
        CoinBumperGadget::new(3),
        Name::new("coin_bumper"),
        Transform::from_scale(Vec3::splat(0.3)),
        Sprite::from_image(image_handle),
        RigidBody::Static,
        Restitution::new(2.0),
        Collider::circle(80.0),
        CollisionEventsEnabled,
        observers![on_coins_spawn_from_bumper],
    )
}


