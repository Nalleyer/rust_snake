use crate::components::{TileMap, TileMapConfig};
use crate::resources::{get_screen_size, Board, Context, Game, State};

use amethyst::{core::Transform, prelude::*, renderer::Camera};

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.write_resource::<Game>().set_state(State::Main);
    }

    fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.write_resource::<Game>().set_state(State::Main);
    }
}
