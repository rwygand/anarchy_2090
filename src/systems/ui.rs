use crate::components::*;
use bevy::prelude::*;

const VIEWPORT_HEIGHT: f32 = 570.0;
const INFO_PANEL_HEIGHT: f32 = 150.0;
const BORDER_WIDTH: f32 = 1.0;

pub fn setup_ui(mut commands: Commands) {
    let root = Node {
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        top: Val::Px(VIEWPORT_HEIGHT),
        width: Val::Px(1260.0),
        height: Val::Px(INFO_PANEL_HEIGHT - 20.0),
        flex_direction: FlexDirection::Column,
        ..default()
    };

    commands.spawn((root, children![stats_bar_bundle(), info_panel_bundle()]));
}

fn stats_bar_bundle() -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(-8.0),
            left: Val::Px(50.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(20.0),
            align_items: AlignItems::Center,
            padding: UiRect::left(Val::Px(16.0)),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        ZIndex(1),
        children![health_bundle(), stamina_bundle(), cyberload_bundle()],
    )
}

fn health_bundle() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        children![
            (
                Text::new("Health:"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                HealthLabel,
            ),
            (
                Text::new(StatDisplay::new(0, 0).format_values()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                StatDisplay::new(0, 0),
                HealthStat,
            )
        ],
    )
}

fn stamina_bundle() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        children![
            (
                Text::new("Stamina:"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                StaminaLabel,
            ),
            (
                Text::new(StatDisplay::new(0, 0).format_values()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                StatDisplay::new(0, 0),
                StaminaStat,
            )
        ],
    )
}

fn cyberload_bundle() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        children![
            (
                Text::new("Load:"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 0.0, 1.0)),
                LoadLabel,
            ),
            (
                Node {
                    padding: UiRect::right(Val::Px(16.0)),
                    ..default()
                },
                Text::new(StatDisplay::new(0, 0).format_values()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                StatDisplay::new(0, 0),
                LoadStat,
            )
        ],
    )
}

fn info_panel_bundle() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(BORDER_WIDTH)),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::BLACK),
        ZIndex(0),
        InfoPanelBorder,
    )
}

fn color_for_current_stat(current: i32, max: i32) -> Color {
    let pct = current as f32 / max as f32;
    if pct > 0.75 {
        Color::srgb(0.0, 1.0, 0.0)
    } else if pct < 0.25 {
        Color::srgb(1.0, 0.0, 0.0)
    } else {
        Color::srgb(1.0, 1.0, 0.0)
    }
}

pub fn update_health_display(
    mut health_query: Query<(&mut Text, &mut TextColor, &mut StatDisplay), With<HealthStat>>,
    mut stamina_query: Query<
        (&mut Text, &mut TextColor, &mut StatDisplay),
        (With<StaminaStat>, Without<HealthStat>),
    >,
    mut load_query: Query<
        (&mut Text, &mut TextColor, &mut StatDisplay),
        (With<LoadStat>, Without<HealthStat>, Without<StaminaStat>),
    >,
    player_query: Query<&Stats, With<Player>>,
) {
    let Ok(stats) = player_query.single() else {
        return;
    };

    // Update health
    if let Ok((mut text, mut color, mut display)) = health_query.single_mut() {
        display.current = stats.current_health;
        display.max = stats.health;
        color.0 = color_for_current_stat(stats.current_health, stats.health);
        text.0 = display.format_values();
    }

    // Update stamina
    if let Ok((mut text, mut color, mut display)) = stamina_query.single_mut() {
        display.current = stats.current_stamina;
        display.max = stats.stamina;
        color.0 = color_for_current_stat(stats.current_stamina, stats.stamina);
        text.0 = display.format_values();
    }

    // Update load
    if let Ok((mut text, mut color, mut display)) = load_query.single_mut() {
        display.current = stats.current_load;
        display.max = stats.load;
        color.0 = color_for_current_stat(stats.current_load, stats.load);
        text.0 = display.format_values();
    }
}

pub fn constrain_camera_to_viewport(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    map: Res<MapDimensions>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let half_viewport_width = 640.0 - BORDER_WIDTH;
    let half_viewport_height = (VIEWPORT_HEIGHT / 2.0) - BORDER_WIDTH;

    let map_half_width = (map.width as f32 * map.tile_size) / 2.0;
    let map_half_height = (map.height as f32 * map.tile_size) / 2.0;

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
