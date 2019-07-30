use crate::components::{TileMap, TileMapConfig};
use crate::resources::{get_screen_size, Board, Context};

use amethyst::{core::Transform, prelude::*, renderer::Camera};

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) { }
}