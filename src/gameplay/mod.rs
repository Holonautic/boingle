use crate::gameplay::game_states::*;
use crate::gameplay::systems::*;
use bevy::prelude::*;
use crate::gadgets::systems::gravity_inverse_field_system;

pub mod components;
pub mod events;
pub mod game_states;
mod systems;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, basic_setup);
        app.add_systems(
            Update,
            widget_placement_system.run_if(in_state(LevelState::PlaceWidget)),
        );
        app.add_systems(
            Update,
            increase_power_gauge_system.run_if(in_state(LevelState::ShootBall)),
        );
        app.add_systems(
            Update,
            ball_left_play_area_system.run_if(in_state(LevelState::BallBouncing)),
        );
        app.add_systems(
            Update,
            destroy_when_standing_still_system.run_if(in_state(LevelState::BallBouncing)),
        );
        app.add_systems(FixedUpdate, draw_trajectory_system);
        app.add_systems(FixedPostUpdate, clamp_max_ball_velocity);
        app.add_systems(
            Update,
            decay_over_time_system
                .run_if(in_state(LevelState::PlaceWidget).or(in_state(LevelState::BallBouncing))),
        );

        app.add_systems(OnExit(LevelState::WidgetSelection), reactivate_gadgets);
        app.add_systems(OnEnter(LevelState::LevelStart), restarting_level);
        app.add_systems(OnEnter(LevelState::EndOfRound), end_of_round_system);
        app.add_systems(Update, gravity_inverse_field_system);

        app.add_systems(OnExit(LevelState::Shop), on_exit_shop);

        app.add_observer(on_gadget_card_selected);
        app.add_observer(on_coin_collected);

        app.add_observer(on_gadget_deactivated_added);
        app.add_observer(on_gadget_deactivated_removed);
        app.add_observer(on_click_on_shop_card_system);
        app.add_observer(on_place_coins_request_system);
    }
}
