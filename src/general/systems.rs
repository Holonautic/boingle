use crate::general::components::*;
use crate::general::resources::GameCursor;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub(super) fn detect_cursor_position(
    mut game_cursor: ResMut<GameCursor>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.into_inner();

    let window = q_window;

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        game_cursor.position = Vec3::new(world_position.x, world_position.y, 0.0);
    }
}
