pub mod components;
pub mod resources;
pub mod systems;

use crate::gadgets::components::*;
use crate::gadgets::resources::GameResources;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use crate::gadgets::systems::*;


// pub fn large_block_bundle(gadget_image_resource: &GameResources) -> impl Bundle {
//     let image_handle = gadget_image_resource
//         .gadget_images
//         .get(&GadgetType::Block)
//         .unwrap()
//         .clone();
//     (
//         Name::new("block_large"),
//         Transform::from_scale(Vec3::splat(0.5)),
//         Sprite::from_image(image_handle),
//         RigidBody::Static,
//         Restitution::new(0.7),
//         Collider::rectangle(125.0, 260.0),
//     )
// }




