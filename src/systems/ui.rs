use crate::components::*;
use bevy::prelude::*;

const VIEWPORT_HEIGHT: f32 = 570.0;
const INFO_PANEL_HEIGHT: f32 = 150.0;
const BORDER_WIDTH: f32 = 1.0;

pub fn setup_ui(mut commands: Commands) {
    // Viewport border (top box)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(VIEWPORT_HEIGHT),
            border: UiRect::all(Val::Px(BORDER_WIDTH)),
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::NONE), // Transparent so map shows through
        ViewportBorder,
        ZIndex(1000), // Render UI on top
    ));

    // Info panel border (bottom box)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(VIEWPORT_HEIGHT),
            width: Val::Percent(100.0),
            height: Val::Px(INFO_PANEL_HEIGHT),
            border: UiRect::all(Val::Px(BORDER_WIDTH)),
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::BLACK),
        InfoPanelBorder,
    ));
}

pub fn constrain_camera_to_viewport(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    map: Res<MapDimensions>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    // Clamp camera position to keep map within viewport
    let half_viewport_width = 640.0 - BORDER_WIDTH; // Half of 1280
    let half_viewport_height = (VIEWPORT_HEIGHT / 2.0) - BORDER_WIDTH;

    let map_half_width = (map.width as f32 * map.tile_size) / 2.0;
    let map_half_height = (map.height as f32 * map.tile_size) / 2.0;

    // Offset camera to account for the info panel at bottom
    let y_offset = -(INFO_PANEL_HEIGHT / 2.0);

    camera_transform.translation.x = camera_transform
        .translation
        .x
        .max(-map_half_width + half_viewport_width)
        .min(map_half_width - half_viewport_width);

    camera_transform.translation.y = (camera_transform.translation.y - y_offset)
        .max(-map_half_height + half_viewport_height)
        .min(map_half_height - half_viewport_height)
        + y_offset;
}
