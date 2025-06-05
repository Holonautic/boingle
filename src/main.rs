mod gadgets;
mod gameplay;
mod general;

use crate::gadgets::resources::GadgetImageResource;
use crate::gadgets::*;
use crate::gameplay::GameplayPlugin;
use crate::gameplay::components::*;
use crate::general::GeneralPlugin;
use crate::general::components::MainCamera;
use avian2d::PhysicsPlugins;
use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rand::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use gameplay::game_states::*;
use std::f32::consts::TAU;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    });
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(SimpleSubsecondPlugin::default());
    app.add_plugins(EntropyPlugin::<WyRand>::default());
    app.add_plugins(Shape2dPlugin::default());
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(PhysicsPickingPlugin::default());
    app.add_plugins(PhysicsDebugPlugin::default());

    app.insert_resource(Gravity(Vector::NEG_Y * 9.81 * 100.0));
    app.insert_resource(GadgetImageResource::default());

    app.add_plugins(GeneralPlugin);
    app.add_plugins(GameplayPlugin);

    app.add_systems(Update, check_loading_state);

    //game states
    app.insert_state(AppState::Loading);
    app.add_sub_state::<LevelState>();

    app.add_systems(Startup, main_setup);

    app.add_systems(Update, greet);
    app.add_systems(Update, spawn_ball);

    app.run()
}

#[derive(Component)]
struct DestroyOnHot;

#[hot(rerun_on_hot_patch = true)]
pub fn main_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    previous_setup: Query<Entity, With<DestroyOnHot>>,
    mut gadget_resource: ResMut<GadgetImageResource>,
) {
    gadget_resource.setup(&asset_server);
    for entity in previous_setup.iter() {
        commands.entity(entity).despawn();
    }

    commands.spawn((DestroyOnHot, Name::new("main camera"), MainCamera, Camera2d));

    let power = 0.0;

    commands
        .spawn((
            DestroyOnHot,
            BallSpitter {
                power: 0.0,
                max_power: 1500.0,
                gain: 1000.0,
                is_increasing_power: false,
            },
            Transform::from_translation(Vec3::new(500.0, -250.0, 0.0))
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, TAU / 14.0)),
            Collider::rectangle(25.0, 50.0),
            RigidBody::Static,
            Mesh2d(meshes.add(Rectangle::new(25.0, 50.0))),
            MeshMaterial2d(materials.add(Color::from(tailwind::BLUE_500))),
            children![(
                IndicatorGauge,
                Transform::from_translation(Vec3::new(0.0, -(1.0 - power) * 25.0, 0.1))
                    .with_scale(Vec3::new(1.0, power, 1.0)),
                Mesh2d(meshes.add(Rectangle::new(25.0, 50.0))),
                MeshMaterial2d(materials.add(Color::from(tailwind::RED_400))),
            )],
        ))
        .observe(
            |trigger: Trigger<Pointer<Pressed>>,
             mut commands: Commands,
             mut q_spitter: Query<&mut BallSpitter>,
             q_balls: Query<Entity, With<PlayerBall>>| {
                for ball_entity in q_balls.iter() {
                    commands.entity(ball_entity).despawn();
                }
                let mut spitter = q_spitter.get_mut(trigger.target).unwrap();
                spitter.is_increasing_power = true;
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>, mut q_spitter: Query<&mut BallSpitter>| {
                let mut spitter = q_spitter.get_mut(trigger.target).unwrap();
                spitter.is_increasing_power = false;
                spitter.power = 0.0;
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Released>>,
             mut commands: Commands,
             asset_server: Res<AssetServer>,
             mut q_spitter: Query<(&mut BallSpitter, &Transform)>,
             mut next_state: ResMut<NextState<LevelState>>,| {
                let (mut spitter, spitter_transform) = q_spitter.get_mut(trigger.target).unwrap();
                spitter.is_increasing_power = false;
                if spitter.power == 0.0 {
                    return;
                }
                let forward = spitter_transform.rotation * Vec3::Y;
                let forward_2d = forward.truncate();
                commands.spawn((
                    DestroyOnHot,
                    ball_bundle(&asset_server),
                    Transform::from_translation(spitter_transform.translation)
                        .with_scale(Vec3::splat(0.5)),
                    LinearVelocity(forward_2d * spitter.power),
                ));
                next_state.set(LevelState::BallBouncing);
                spitter.power = 0.0;
            },
        );

    // commands.spawn((
    //     DestroyOnHot,
    //     large_block(&asset_server),
    //     Transform {
    //         translation: Vec3::new(0., -150.0, 0.),
    //         rotation: Quat::from_rotation_z(TAU * 0.3),
    //         scale: Vec3::splat(0.5),
    //     },
    // ));
    // commands.spawn((
    //     DestroyOnHot,
    //     large_block(&asset_server),
    //     Transform {
    //         translation: Vec3::new(-550., -150.0, 0.),
    //         rotation: Quat::from_rotation_z(TAU * -0.3),
    //         scale: Vec3::splat(0.5),
    //     },
    // ));
    //
    // commands.spawn((
    //     DestroyOnHot,
    //     large_block(&asset_server),
    //     Transform {
    //         translation: Vec3::new(-550., 0.0, 0.),
    //         rotation: Quat::from_rotation_z(TAU * -0.0),
    //         scale: Vec3::splat(0.5),
    //     },
    // ));
    //
    // commands.spawn((
    //     DestroyOnHot,
    //     large_block(&asset_server),
    //     Transform {
    //         translation: Vec3::new(-550., 250.0, 0.),
    //         rotation: Quat::from_rotation_z(TAU * -0.1),
    //         scale: Vec3::splat(0.5),
    //     },
    // ));
    //
    // commands.spawn((
    //     DestroyOnHot,
    //     bumper(&asset_server),
    //     Transform {
    //         translation: Vec3::new(-190., -150.0, 0.),
    //         rotation: Quat::from_rotation_z(TAU * -0.3),
    //         scale: Vec3::splat(1.0),
    //     },
    // ));
    //
    // commands.spawn((
    //     DestroyOnHot,
    //     bumper(&asset_server),
    //     Transform {
    //         translation: Vec3::new(-215., -250.0, 0.),
    //         rotation: Quat::from_rotation_z(TAU * -0.3),
    //         scale: Vec3::splat(1.0),
    //     },
    // ));
}

fn check_loading_state(
    gadget_resource: Res<GadgetImageResource>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if gadget_resource.gadget_images.len() > 0 {
        next_state.set(AppState::InGame);
    }
}

#[hot]
fn greet(time: Res<Time>) {
    info_once!(
        "Hello from a hotpatched system! Try changing this string while the app is running! Patched at t = {} s",
        time.elapsed_secs()
    );
}

#[derive(Component, Debug, Reflect)]
pub struct PlayerBall;

#[hot]
fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    ball_query: Query<Entity, With<PlayerBall>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    for entity in ball_query.iter() {
        commands.entity(entity).despawn();
    }

    let ball_image = asset_server.load("sprites/ball_blue_small.png");
    commands.spawn((
        DestroyOnHot,
        Name::new("ball_blue_small"),
        PlayerBall,
        Sprite::from_image(ball_image),
        Transform {
            translation: Vec3::new(0., 300.0, 0.),
            rotation: Quat::from_rotation_z(TAU * 0.25),
            scale: Vec3::splat(0.5),
        },
        RigidBody::Dynamic,
        Restitution::new(0.99),
        Collider::circle(30.0),
    ));
}
