use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct GameCursor {
    pub position: Vec3
}