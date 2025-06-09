mod cards;
mod experiments;
mod gadgets;
mod game_ui;
mod gameplay;
mod general;

use crate::experiments::ExperimentsPlugin;
use crate::gadgets::components::{Block, GadgetType, GravityReverseField, SquareBlock, WideBlock};
use crate::gadgets::resources::GameResources;
use crate::game_ui::GameUiPlugin;
use crate::gameplay::GameplayPlugin;
use crate::gameplay::components::*;
use crate::general::GeneralPlugin;
use crate::general::components::*;
use avian2d::PhysicsPlugins;
use avian2d::math::Vector;
use avian2d::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_easings::EasingsPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rand::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use gameplay::game_states::*;
use rand::Rng;
use std::f32::consts::TAU;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Boingle".to_string(),
                    // Bind to canvas included in `index.html`
                    canvas: Some("#bevy".to_owned()),
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5 and Ctrl+R
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
    );

    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    });
    app.add_plugins(SimpleSubsecondPlugin::default());
    app.add_plugins(EntropyPlugin::<WyRand>::default());
    app.add_plugins(Shape2dPlugin::default());
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(PhysicsPickingPlugin::default());
    // app.add_plugins(PhysicsDebugPlugin::default());
    // app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(EasingsPlugin::default());
    app.insert_resource(Gravity(Vector::NEG_Y * 9.81 * 100.0));
    app.insert_resource(GameResources::default());

    app.add_plugins(GeneralPlugin);
    app.add_plugins(GameplayPlugin);
    app.add_plugins(GameUiPlugin);
    app.add_plugins(ExperimentsPlugin);

    //game states
    app.insert_state(AppState::Loading);
    app.add_sub_state::<LevelState>();
    app.add_sub_state::<MenuState>();

    app.add_systems(OnEnter(AppState::Loading), load_assets);
    app.add_systems(OnEnter(AppState::Startup), startup_setup);
    app.add_systems(OnEnter(AppState::InGame), main_game_setup);

    app.add_systems(Update, greet);

    app.run()
}

#[derive(Component)]
struct DestroyOnHot;

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut gadget_resource: ResMut<GameResources>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    gadget_resource.setup(&asset_server);
    next_state.set(AppState::Startup);
}

pub fn startup_setup(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    commands.spawn((Name::new("main camera"), MainCamera, Camera2d));
    next_state.set(AppState::Menu);
}

#[hot(rerun_on_hot_patch = true)]
pub fn main_game_setup(
    mut commands: Commands,
    previous_setup: Query<Entity, With<DestroyOnHot>>,
    mut rng: GlobalEntropy<WyRand>,
    game_resources: Res<GameResources>,
) {
    for entity in previous_setup.iter() {
        commands.entity(entity).try_despawn();
    }

    let x_position = game_resources.play_area.x - 0.0;
    let y_min_max = game_resources.play_area.y - 0.0;
    let y_position = rng.random_range(-y_min_max..=y_min_max);

    let base_angle = 90.0;
    let jitter = rng.random_range(-60.0..=60.0);
    // let jitter = 0.0;

    let angle = base_angle + jitter;

    commands.spawn((
        DestroyOnHot,
        BallCannon::bundle(),
        Transform::from_xyz(x_position, y_position, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(angle))),
    ));

}

#[hot]
fn greet(time: Res<Time>) {
    info_once!(
        "Hello from a hotpatched system! Try changing this string while the app is running! Patched at t = {} s",
        time.elapsed_secs()
    );
}
