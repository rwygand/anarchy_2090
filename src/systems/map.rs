use crate::components::{BlocksMovement, MapDimensions, Wall};
use crate::helpers::grid_to_world_position;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn generate_map(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let map_dims = MapDimensions {
        width: 80,
        height: 50,
        ..default()
    };

    // Create a 16x16 black sprite image
    let image_size = map_dims.tile_size as u32;
    let pixel_count = (image_size * image_size * 4) as usize; // RGBA
    let mut pixels = vec![0u8; pixel_count];

    // Fill with black color (R=0, G=0, B=0, A=255)
    for i in (0..pixel_count).step_by(4) {
        pixels[i] = 0; // R
        pixels[i + 1] = 0; // G
        pixels[i + 2] = 0; // B
        pixels[i + 3] = 255; // A
    }

    let image = Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: image_size,
            height: image_size,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &pixels,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    let texture_handle = images.add(image);

    let map_size = TilemapSize {
        x: map_dims.width,
        y: map_dims.height,
    };
    let tile_size = TilemapTileSize {
        x: map_dims.tile_size,
        y: map_dims.tile_size,
    };
    let grid_size = TilemapGridSize {
        x: map_dims.tile_size,
        y: map_dims.tile_size,
    };
    let map_type = TilemapType::Square;

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        -(map_size.x as f32 * tile_size.x) / 2.0,
        -(map_size.y as f32 * tile_size.y) / 2.0,
        0.0,
    ));

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type,
        transform,
        ..default()
    });

    // Spawn perimeter walls
    for x in 0..map_dims.width {
        for y in 0..map_dims.height {
            // Check if position is on perimeter
            if x == 0 || x == map_dims.width - 1 || y == 0 || y == map_dims.height - 1 {
                let wall_pos = TilePos { x, y };
                let world_pos = grid_to_world_position(&wall_pos, &map_dims, 10.0);

                commands.spawn((
                    Text2d::new("#"),
                    TextFont {
                        font_size: map_dims.tile_size,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)), // Gray walls
                    Transform::from_translation(world_pos),
                    Wall,
                    BlocksMovement,
                    wall_pos,
                ));
            }
        }
    }

    commands.insert_resource(map_dims);
}
