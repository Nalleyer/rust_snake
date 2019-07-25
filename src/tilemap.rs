use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{transform::TransformBundle, Transform},
    ecs::prelude::{
        Component, DenseVecStorage, Entity, ReadExpect, Resources, SystemData, VecStorage,
    },
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        pass::DrawShadedDesc,
        rendy::{
            factory::Factory,
            graph::{
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{format::Format, image},
        },
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        types::DefaultBackend,
        GraphCreator, RenderingSystem, Texture,
    },
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};
use ron::de::from_str;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TileDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tile {
    ttype: u8,
    direction: TileDirection,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            ttype: 0,
            direction: TileDirection::Up,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TileMapConfig {
    tile_width: usize,
    tile_height: usize,
    size_x: usize,
    size_y: usize,
}

impl TileMapConfig {
    fn from_path(path: &str) -> TileMapConfig {
        let file_content = fs::read_to_string(path).expect("reading tilemap setting");
        from_str(&file_content).expect("parsing tile config")
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct TileMap {
    tiles: Vec<Tile>, // list of tile type
    entities: Vec<Entity>,
    tile_set: Handle<SpriteSheet>,
}

impl TileMap {
    pub fn new(
        world: &mut World,
        asset_path: &str,
        asset_config: &str,
        config_path: &str,
    ) -> TileMap {
        let config = TileMapConfig::from_path(config_path);
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(asset_path, ImageFormat::default(), (), &texture_storage)
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                asset_config,
                SpriteSheetFormat(texture_handle),
                (),
                &sprite_sheet_store,
            )
        };

        let mut entities: Vec<Entity> = vec![];
        let mut tiles: Vec<Tile> = vec![];

        for x in 0..config.size_x {
            for y in 0..config.size_y {
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: 0, //default
                };

                let mut transform = Transform::default();

                transform.set_translation_xyz(
                    (x as f32 + 0.5) * config.tile_width as f32,
                    (y as f32 + 0.5) * config.tile_height as f32,
                    0.0,
                );

                let entity = world
                    .create_entity()
                    .with(transform)
                    .with(sprite_render.clone())
                    .build();
                entities.push(entity);
                tiles.push(Tile::default());
            }
        }

        TileMap {
            tiles: tiles,
            entities: entities,
            tile_set: sprite_sheet_handle,
        }
    }
}
