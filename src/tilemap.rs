use amethyst::{
    core::transform::TransformBundle,
    ecs::prelude::{ReadExpect, Resources, SystemData, Component, DenseVecStorage, VecStorage},
    prelude::*,
    renderer::{
        pass::DrawShadedDesc,
        rendy::{
            factory::Factory,
            graph::{
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{format::Format, image},
        },
        types::DefaultBackend,
        GraphCreator, RenderingSystem,
    },
    utils::application_root_dir,
    window::{ScreenDimensions, Window, WindowBundle},
};

#[derive(Debug, Eq, PartialEq)]
pub enum TileDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Tile {
    ttype: u8,
    direction: TileDirection,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
struct TileMap {
    tiles: Vec<Tile>, // list of tile type
}