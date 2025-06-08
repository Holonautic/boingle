use bevy::prelude::*;
use crate::game_ui::systems::*;
use crate::gameplay::game_states::{AppState, LevelState};

mod systems;
pub mod components;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui);
        app.add_systems(Update, update_ui);
        app.add_systems(Update, button_system);
        
        app.add_systems(OnEnter(LevelState::GameOver), spawn_level_over_ui);
        app.add_systems(OnEnter(LevelState::PlaceWidget), show_widget_selection);
        app.add_systems(OnEnter(LevelState::Shop), create_shop_ui);

        app.add_observer(widget_selection_ui_despawn);


    }
}