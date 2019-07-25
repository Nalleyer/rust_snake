use crate::loader;
use crate::tilemap::{TileMap};

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Transform,
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        Camera, Texture,
    },
    window::{ScreenDimensions, Window, WindowBundle},
};

pub struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<TileMap>();
        initialise_camera(world);

        // let texture_handle = loader::load_texture("assets/snake.png", world);

        // let sprite_sheet_handle = load_sprite_sheet(world);

        // init_image(world, &sprite_sheet_handle);

        let tilemap = {
            TileMap::new(world, "assets/snake.png", "assets/snake.ron", "resources/assets/tileset.ron")
        };

        world.create_entity()
            .with(Transform::default())
            .with(tilemap)
            .build();
        println!("init tile");
    }
}

// for test
fn init_image(world: &mut World, sprite_sheet_handle: &Handle<SpriteSheet>) {
    // Add a transform component to give the image a position
    let mut transform = Transform::default();
    transform.set_translation_x(400.0);
    transform.set_translation_y(300.0);
    // Flip horizontally
    // transform.set_rotation_y_axis(f32::pi());
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite_render.clone()) // Use the texture handle as a component
        .build();
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let (width, height) = {
        let dimensions = world.read_resource::<ScreenDimensions>();

        let width = dimensions.width();
        let height = dimensions.height();
        (width, height)
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "assets/snake.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // ...

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "assets/snake.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
