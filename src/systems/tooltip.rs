use crate::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn track_cursor(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };

    cursor_pos.screen = window.cursor_position();

    if let Some(screen_pos) = cursor_pos.screen {
        let Ok((camera, camera_transform)) = camera_query.single() else {
            return;
        };

        cursor_pos.world = camera
            .viewport_to_world_2d(camera_transform, screen_pos)
            .ok();
    }
}

pub fn setup_tooltip(mut commands: Commands) {
    info!("Setting up tooltip entity");
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
            BorderColor::all(Color::WHITE),
            Visibility::Visible, // Start visible for debugging
            ZIndex(1000),
            TooltipText,
        ))
        .with_child((
            Text::new("Test Tooltip Text"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
}

pub fn update_tooltip(
    cursor_pos: Res<CursorPosition>,
    entity_query: Query<(&Transform, &Named), Without<Camera2d>>,
    mut tooltip_query: Query<(Entity, &mut Node, &mut Visibility), With<TooltipText>>,
    mut text_query: Query<&mut Text>,
    map: Res<MapDimensions>,
    children_query: Query<&Children>,
) {
    let Ok((tooltip_entity, mut node, mut visibility)) = tooltip_query.single_mut() else {
        warn!("Tooltip entity not found!");
        return;
    };

    let Some(world_pos) = cursor_pos.world else {
        *visibility = Visibility::Hidden;
        return;
    };

    let Some(screen_pos) = cursor_pos.screen else {
        *visibility = Visibility::Hidden;
        return;
    };

    let hover_threshold = map.tile_size / 2.0;
    let mut hovered_entity: Option<&Named> = None;

    for (transform, named) in entity_query.iter() {
        let distance = transform.translation.truncate().distance(world_pos);
        if distance <= hover_threshold {
            hovered_entity = Some(named);
            break;
        }
    }

    if let Some(named) = hovered_entity {
        *visibility = Visibility::Visible;

        // Update text through children query
        if let Ok(children) = children_query.get(tooltip_entity) {
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = named.0.clone();
                }
            }
        }

        node.left = Val::Px(screen_pos.x + 15.0);
        node.top = Val::Px(screen_pos.y + 15.0);
    } else {
        *visibility = Visibility::Hidden;
    }
}
