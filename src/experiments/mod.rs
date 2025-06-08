use crate::cards::components::ShopCardType;
use crate::gameplay::game_states::AppState;
use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

pub struct ExperimentsPlugin;

impl Plugin for ExperimentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Experiments), experiments_setup);
        app.add_systems(Startup, register_components);
    }
}

#[hot(rerun_on_hot_patch = true)]
fn register_components(registry: Res<AppTypeRegistry>) {
    let mut registry = registry.write();
    registry.register::<ShopCardType>();
}

#[derive(Component)]
struct ExperimentSetupDestroyOnReload;

#[hot(rerun_on_hot_patch = true)]
pub fn experiments_setup(
    mut commands: Commands,
    query: Query<Entity, With<ExperimentSetupDestroyOnReload>>,
) {
    for entity in query.iter() {
        info!("destroying something");
        commands.entity(entity).despawn();
    }
    info!("Experiments setup");
    commands.spawn((
        ExperimentSetupDestroyOnReload,
        Transform::from_xyz(-200.0, 50.0, 10.0),
        ShopCardType::SquareBlockCard,
    ));
}
