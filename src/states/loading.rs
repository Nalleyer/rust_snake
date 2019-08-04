use crate::components::{TileMap, TileMapConfig};
use crate::resources::{get_screen_size, Board, Context, Game, State};
use crate::states::MainState;

use amethyst::{core::Transform, prelude::*, renderer::Camera};

pub struct LoadingState {
    load_complete: bool,
}

impl Default for LoadingState {
    fn default() -> Self {
        LoadingState {
            load_complete: false,
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<TileMap>();
        initialise_camera(world);

        world.add_resource(Context::new());
        world.add_resource(Game::new(State::Loading));

        let tile_config = TileMapConfig::from_path("resources/assets/tileset.ron");
        let board = Board::new(tile_config.size_x, tile_config.size_y);

        world.add_resource(board);

        let tilemap = { TileMap::new(world, "assets/snake.png", "assets/snake.ron", &tile_config) };
        let (width, height) = get_screen_size(world);
        let mut transform = Transform::default();

        transform.set_translation_xyz((width * 0.5) as f32, (height * 0.5) as f32, 0.0);

        world.create_entity().with(transform).with(tilemap).build();
        self.load_complete = true;
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match &self.load_complete {
            false => Trans::None,
            true => Trans::Switch(Box::new(MainState {})),
        }
    }

    fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world
            .write_resource::<Game>()
            .set_state(State::Loading);
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
