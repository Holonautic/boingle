pub mod components;
pub mod resources;
mod systems;

use crate::gadgets::components::*;
use crate::gadgets::resources::GameResources;
use crate::gameplay::components::{BallSpawnPoint, HelpTextFor, HelperText};
use avian2d::prelude::*;
use bevy::asset::AssetServer;
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


pub fn ball_spawn_point(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> impl Bundle {
    let mesh_handle = meshes.add(Annulus::new(25.0, 30.0));
    let material_handle = materials.add(Color::from(tailwind::BLUE_500));
    (
        Name::new("ball_spawn_point"),
        BallSpawnPoint,
        Mesh2d(mesh_handle),
        MeshMaterial2d(material_handle),
        Collider::circle(25.0),
    )
}



pub fn spawn_ball_spawner(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> Entity {
    commands
        .spawn((
            ball_spawn_point(meshes, materials),
            Transform::from_xyz(-50.0, 150.0, 0.0),
        ))
        .observe(|trigger: Trigger<Pointer<Over>>, mut commands: Commands| {
            commands.spawn((
                Text2d::new("Click To Spawn Ball"),
                Transform::from_translation(Vec3::Y * 60.0),
                ChildOf(trigger.target),
                HelpTextFor(trigger.target),
            ));
        })
        .observe(
            |trigger: Trigger<Pointer<Out>>,
             mut commands: Commands,
             q_help_text: Query<&HelperText>| {
                let helper_texts = q_help_text.get(trigger.target).unwrap();
                for text_entity in helper_texts.get_helper_texts().iter() {
                    commands.entity(*text_entity).despawn();
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Click>>,
             mut commands: Commands,
             asset_server: Res<AssetServer>,
             q_transform: Query<&Transform>| {
                let mut transform = q_transform.get(trigger.target).unwrap().clone();
                transform.scale = Vec3::splat(0.5);
                commands.spawn((PlayerBall, transform));
            },
        )
        .id()
}
