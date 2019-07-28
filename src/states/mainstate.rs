use crate::components::{TileMap, TileMapConfig};
use crate::resources::{get_screen_size, Board, Context};

use amethyst::{core::Transform, prelude::*, renderer::Camera};

pub struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<TileMap>();
        initialise_camera(world);

        world.add_resource(Context::new());

        let tile_config = TileMapConfig::from_path("resources/assets/tileset.ron");
        let board = Board::new(tile_config.size_x, tile_config.size_y);

        world.add_resource(board);

        let tilemap = { TileMap::new(world, "assets/snake.png", "assets/snake.ron", &tile_config) };
        let (width, height) = get_screen_size(world);
        let mut transform = Transform::default();

        transform.set_translation_xyz((width * 0.5) as f32, (height * 0.5) as f32, 0.0);

        world.create_entity().with(transform).with(tilemap).build();
        println!("init tile");
    }
}

fn initialise_camera(world: &mut World) {
    let (width, height) = get_screen_size(world);

    let mut transform = Transform::default();
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}
