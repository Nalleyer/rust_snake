use crate::components::{TileMap, TileMapConfig};
use crate::resources::{get_screen_size, Board, Context, Game, State};

use amethyst::{core::Transform, prelude::*, renderer::Camera,
    ui::{
        UiCreator,
        UiFinder,
    },
};

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.write_resource::<Game>().set_state(State::Main);

        if let Some(entity) = world.exec(|finder: UiFinder| finder.find("loading")) {
            world.delete_entity(entity).expect("deleting loading ui");
        }
    }

    fn on_resume(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.write_resource::<Game>().set_state(State::Main);
    }
}
