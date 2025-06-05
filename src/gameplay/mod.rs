use bevy::prelude::*;
use crate::gameplay::game_states::{AppState, LevelState};
use crate::gameplay::systems::*;
use crate::main_setup;

pub mod components;
mod systems;
pub mod game_states;
mod events;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(Startup, basic_setup);
        app.add_systems(Update, widget_placement_system.run_if(in_state(LevelState::PlaceWidget)));
        app.add_systems(Update, increase_power_gauge_system.run_if(in_state(LevelState::ShootBall)));
        app.add_systems(Update, remove_all_placed_gadgets_system);
        app.add_systems(Update, ball_left_play_area_system.run_if(in_state(LevelState::BallBouncing)));
        app.add_systems(FixedPostUpdate, clamp_max_ball_velocity);

        app.add_systems(OnEnter(LevelState::PlaceWidget), show_widget_selection);

        app.add_observer(on_gadget_card_selected);

    }
}