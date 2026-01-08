use bevy::camera::Camera2d;
use bevy::prelude::{Commands, Query, Transform, With, Without};
use crate::components::Player;


pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) =
        (player_query.single(), camera_query.single_mut()) {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}