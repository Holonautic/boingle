use bevy::prelude::*;
use crate::gameplay::game_states::*;
use crate::gameplay::systems::*;

pub mod components;
mod systems;
pub mod game_states;
pub mod events;

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
        app.add_systems(OnExit(LevelState::PlaceWidget), reactivate_gadgets);
        app.add_systems(OnEnter(LevelState::LevelStart), restarting_level);
        app.add_systems(OnEnter(LevelState::EndOfRound), end_of_round_system);

        app.add_systems(OnEnter(LevelState::Shop), on_enter_shop);

        app.add_systems(OnExit(LevelState::Shop), on_exit_shop);

        app.add_observer(on_gadget_card_selected);
        app.add_observer(on_coin_collected);
        
        app.add_observer(on_gadget_deactivated_added);
        app.add_observer(on_gadget_deactivated_removed);



    }
}
