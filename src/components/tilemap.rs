use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, Transform},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        Texture,
    },
};
use ron::de::from_str;
use serde::Deserialize;
use std::fs;

use crate::resources::get_scale;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tile;

impl Default for Tile {
    fn default() -> Self {
        Tile {}
    }
}

#[derive(Debug, Deserialize)]
pub struct TileMapConfig {
    pub tile_width: usize,
    pub tile_height: usize,
    pub size_x: usize,
    pub size_y: usize,
}

impl TileMapConfig {
    pub fn from_path(path: &str) -> TileMapConfig {
        let file_content = fs::read_to_string(path).expect("reading tilemap setting");
        from_str(&file_content).expect("parsing tile config")
    }
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct TileMap {
    pub entities: Vec<Entity>,
    pub tile_set: Handle<SpriteSheet>,
    len: usize,
}

impl TileMap {
    pub fn new(
        world: &mut World,
        asset_path: &str,
        asset_config: &str,
        config: &TileMapConfig,
    ) -> TileMap {
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

        for x in 0..config.size_x {
            for y in 0..config.size_y {
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: 0, //default
                };

                let mut transform = Transform::default();

                let scale = get_scale(world);
                transform.set_translation_xyz(
                    (x as f32 + 0.5) * scale * config.tile_width as f32,
                    (y as f32 + 0.5) * scale * config.tile_height as f32,
                    0.0,
                );

                transform.set_scale(Vector3::new(scale, scale, scale));

                let entity = world
                    .create_entity()
                    .with(transform)
                    .with(sprite_render.clone())
                    .build();
                entities.push(entity);
            }
        }

        let len = entities.len();
        TileMap {
            entities,
            tile_set: sprite_sheet_handle,
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
