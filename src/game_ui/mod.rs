use bevy::prelude::*;
use crate::game_ui::systems::*;
use crate::gameplay::game_states::{AppState, LevelState, MenuState};

mod systems;
pub mod components;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui);
        app.add_systems(Update, update_ui);
        app.add_systems(Update, button_system);

        app.add_systems(OnEnter(MenuState::MainMenu), show_menu_ui);
        app.add_systems(OnEnter(MenuState::CreditsMenu), show_credits);

        app.add_systems(OnEnter(LevelState::GameOver), spawn_level_over_ui);
        app.add_systems(OnEnter(LevelState::WidgetSelection), show_widget_selection);
        app.add_systems(OnEnter(LevelState::Shop), show_shop_ui);
        app.add_systems(OnEnter(LevelState::ShootBall), on_entering_shoot_ball_state);
        app.add_systems(OnExit(LevelState::ShootBall), on_exit_shoot_ball_state);


        app.add_systems(Update, update_shop_ui.run_if(in_state(LevelState::Shop)));

        app.add_observer(widget_selection_ui_despawn);


    }
}