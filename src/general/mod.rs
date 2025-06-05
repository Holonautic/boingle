use bevy::app::App;
use bevy::prelude::*;
use crate::general::resources::GameCursor;
use crate::general::systems::*;

pub mod resources;
mod systems;
pub mod components;



pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {

        app.init_resource::<GameCursor>();
        app.add_systems(Update, detect_cursor_position);
    }
}